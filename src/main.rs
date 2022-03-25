#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::{fmt, io};
use std::any::Any;
use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::fmt::Write;
use std::fs::File;
use std::io::Read;
use std::mem;
use std::ops::Deref;
use std::panic::panic_any;
use std::rc::Rc;

use tok_flags::{IS_AT_BOL, LF_AFTER, WS_BEFORE};
use token::Token;

use crate::cbuf::CBuf;
use crate::tok_maps::Keywords;
use crate::token::{Ident, SourceLoc, T};
use crate::token::T::HASH_NEWLINE;
use crate::tokenizer::Tokenizer;

mod cbuf;
mod ascii_util;
mod tok_maps;
mod tok_flags;
mod tokenizer;
mod token;
mod tok_printer;


#[derive(Debug, Clone)]
struct TokenBuf {
    tokens: Vec<Rc<Token>>,
}

impl TokenBuf {
    fn new(input: Vec<Token>) -> Self {
        let mut tokens: Vec<Rc<Token>> = Vec::new();
        for t in input {
            tokens.push(Rc::new(t));
        }

        TokenBuf {
            tokens,
        }
    }

    fn new_rc(tokens: Vec<Rc<Token>>) -> Self {
        TokenBuf {
            tokens
        }
    }

    fn new_empty() -> Self {
        TokenBuf {
            tokens: vec![]
        }
    }

    fn push_front(&mut self, t: Rc<Token>) {
        self.tokens.insert(0, t);
    }

    fn is_empty(&self) -> bool {
        return self.tokens.is_empty();
    }

    #[inline]
    fn tok(&self) -> Rc<Token> {
        assert!(!self.is_empty());
        return Rc::clone(&self.tokens.get(0).unwrap());
    }

    fn move_get(&mut self) -> Rc<Token> {
        assert!(!self.is_empty());

        let saved = self.tok();
        self.tokens.remove(0);

        return saved;
    }

    fn move_get_id(&mut self, id: &Rc<RefCell<Ident>>) -> Rc<Token> {
        if self.tok().is_ident(&id) {
            return self.move_get();
        }
        panic!("expected identifier: `{:?}`, but found value: `{}`", id, &self.tok().value);
    }

    fn move_get_tp(&mut self, tp: T) -> Rc<Token> {
        // TODO: remove clone() here, add to_string() impl for T::
        if self.tok().is(tp.clone()) {
            return self.move_get();
        }
        panic!("expected token-type: `{:?}`, but found value: `{}`", &tp, &self.tok().value);
    }

    fn cut_till_newline(&mut self) -> Vec<Rc<Token>> {
        let mut result: Vec<Rc<Token>> = Vec::new();
        while !self.is_empty() {
            let tok = self.move_get();
            if tok.has_newline_after() || tok.is(T::TOKEN_EOF) {
                result.push(tok);
                break;
            }
            result.push(tok);
        }
        return result;
    }
}

#[derive(Debug, Clone)]
struct Macros {
    name: String,
    repl: Vec<Rc<Token>>,
    is_hidden: bool,
}

impl Macros {
    fn new(name: String, mut repl: Vec<Rc<Token>>) -> Self {

        let unhide = Token::new(T::T_SPEC_UNHIDE, name.clone(), SourceLoc::default());
        repl.push(Rc::new(unhide));

        Macros {
            name,
            repl,
            is_hidden: false,
        }
    }

    fn hide(&mut self) {
        self.is_hidden = true;
    }

    fn unhide(&mut self) {
        self.is_hidden = false;
    }

    fn is_hidden(&self) -> bool {
        return self.is_hidden;
    }

    fn clonerepl(&self) -> Vec<Rc<Token>> {
        return self.repl.clone();
    }
}

#[derive(Debug, Clone)]
struct Scan {
    tokens: TokenBuf,
    rescan: TokenBuf,
    symtab: HashMap<String, Macros>,
}

impl Scan {
    fn new(tokens: Vec<Token>) -> Self {
        Scan {
            tokens: TokenBuf::new(tokens),
            rescan: TokenBuf::new_empty(),
            symtab: HashMap::new(),
        }
    }

    fn is_empty(&self) -> bool {
        return self.tokens.is_empty() && self.rescan.is_empty();
    }

    fn push(&mut self,  t :Rc<Token>) {
        self.rescan.push_front(t);
    }

    fn pop_noppdirective(&mut self) -> Rc<Token> {
        if !self.rescan.is_empty() {
            return self.rescan.move_get();
        }
        return self.tokens.move_get();
    }

    fn pop(&mut self) -> Rc<Token> {
        let mut t = self.pop_noppdirective();
        if t.is(T::T_SHARP) && t.is_at_bol() {
            if t.has_newline_after() {
                let loc = &t.loc;
                return Rc::new(Token::new(T::HASH_NEWLINE, "#\n".to_string(), loc.clone()));
            }
            let pp = self.pop_noppdirective();
            let value = pp.value.clone();
            if value == "define" {
                let loc = &t.loc;
                return Rc::new(Token::new(T::HASH_DEFINE, "#define".to_string(), loc.clone()));
            }
            panic!("unimplemented pp-directive: {}", value);
        }
        return t;
    }

    fn define(&mut self) {
        let name = self.pop();
        if !name.is(T::TOKEN_IDENT) {
            panic!("expected macro name, but found: {:?}", name);
        }

        let mut repl: Vec<Rc<Token>> = Vec::new();
        while !self.is_empty() {
            let t = self.pop();
            if t.has_newline_after() || t.is(T::TOKEN_EOF) {
                repl.push(t);
                break;
            }
            repl.push(t);
        }

        let m = Macros::new(name.value.clone(), repl);
        self.symtab.insert(name.value.clone(), m);
    }

    fn get(&mut self) -> Rc<Token> {
        'restart:
        while !self.is_empty() {
            let t = self.pop();

            if t.is(T::HASH_DEFINE) {
                self.define();
                continue;
            }
            if self.unhide(&t) {
                continue 'restart;
            }
            if !t.is(T::TOKEN_IDENT) {
                return t;
            }
            if t.noexpand {
                return t;
            }
            let macros = self.symtab.get(&t.value);
            if macros.is_none() {
                return t;
            }
            if macros.unwrap().is_hidden() {
                let mut noexpand = Token::new_from(t);
                noexpand.noexpand = true;
                return Rc::new(noexpand);
            }

            // if (macros.isObjectLike()) {
            //    replaceSimple(macros, macros.getRepl(), t, null);
            //    continue restart;
            // }

            self.replace_simple(t);
        }
        return Rc::new(Token::new(T::TOKEN_EOF, "<EOF>".to_string(), SourceLoc::default()));
    }

    fn unhide(&mut self, u: &Rc<Token>) -> bool {
        if u.is(T::T_SPEC_UNHIDE) {
            let macros = self.symtab.get_mut(&u.value.clone()).unwrap();
            macros.unhide();
            return true;
        }
        return false;
    }

    // private void replaceSimple(Sym macros, List<Token> repl, Token t, ArgInfo argInfo) {
    //     macros.hide();
    //
    //     List<Token> replacement = repl;
    //     List<Token> res = pasteAll(t, replacement, argInfo);
    //     for (int j = res.size(); --j >= 0;) {
    //       Token tokp = res.get(j);
    //       if (!tokp.is(T.T_SPEC_PLACEMARKER)) {
    //         push(tokp);
    //       }
    //     }
    // }

    fn replace_simple(&mut self, t: Rc<Token>) {
        // we know it's here :)
        let macros = self.symtab.get_mut(&t.value).unwrap();
        macros.hide();

        let rep = macros.repl.clone();
        let mut res = self.paste_all(t, rep);

        while !res.is_empty() {
            let tokp = res.remove(res.len() -1);
            if tokp.is(T::T_SPEC_PLACEMARKER) {
                continue;
            }
            self.push(tokp);
        }
    }

    // private List<Token> pasteAll(Token headToCopy, List<Token> replacement, ArgInfo argInfo) {
    //   List<Token> rv = new LinkedList<Token>();
    //   for (int iter = 0; iter < replacement.size(); iter++) {
    //
    //     Token tok = replacement.get(iter);
    //
    //       Token ntok = new Token(tok);
    //       ntok.setLocation(new SourceLocation(headToCopy.getLocation()));
    //       rv.add(ntok);
    //
    //   }
    //
    //   if (!rv.isEmpty()) {
    //     Token first = rv.get(0);
    //     first.setLeadingWhitespace(headToCopy.hasLeadingWhitespace());
    //   }
    //
    //   return rv;
    // }

    fn paste_all(&mut self, head: Rc<Token>, repl: Vec<Rc<Token>>) -> Vec<Rc<Token>> {
        let mut rv: Vec<Rc<Token>> = Vec::new();

        for tok in repl {
            let mut ntok = Token::new_from(tok);
            let nloc = head.loc.clone();
            ntok.loc = nloc;
            rv.push(Rc::new(ntok));
        }

        return rv;
    }
}

fn main() {
    let filename = "./resources/test_data/test1.txt".to_string();

    // this one we will use through the whole program
    let keywords = Keywords::new();

    // this one we will move to tokenizer, we do not need to use it by hand.
    // we do not interested with this hash-table, its purpose to make all identifiers
    // unique, that's it.
    let identifiers = tok_maps::make_id_map(&keywords);

    let mut tokenizer = Tokenizer::new_from_file(filename, identifiers);
    let tokens = tokenizer.tokenize();
    let mut scan = Scan::new(tokens);

    while !scan.is_empty() {
        let t = scan.get();
        println!("{}", &t.value);
    }

    println!("\n:ok:\n");
}






























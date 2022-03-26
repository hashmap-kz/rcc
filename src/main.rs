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
use crate::shared::{shared_ptr, shared_vec};
use crate::T::{HASH_NEWLINE, TOKEN_EOF};
use crate::tok_maps::Keywords;
use crate::token::{Ident, SourceLoc, Sym, T};
use crate::tokenizer::Tokenizer;

mod cbuf;
mod ascii_util;
mod tok_maps;
mod tok_flags;
mod tokenizer;
mod token;
mod tok_printer;
mod shared;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Scan {
    tokens: shared_vec<Token>,
    rescan: shared_vec<Token>,
    offset: usize,
    size: usize,
}

impl Scan {
    fn new(tokens: shared_vec<Token>) -> Self {
        let size = tokens.len();
        Scan {
            tokens,
            rescan: shared_vec::new(),
            offset: 0,
            size,
        }
    }

    fn no_tokens_anymore(&self) -> bool {
        return self.offset >= self.size;
    }

    fn is_empty(&self) -> bool {
        return self.no_tokens_anymore() && self.rescan.is_empty();
    }

    fn push(&mut self, t: shared_ptr<Token>) {
        self.rescan.push_back(t);
    }

    fn pop_noppdirective(&mut self) -> shared_ptr<Token> {
        if !self.rescan.is_empty() {
            return self.rescan.pop_back();
        }
        let tok = self.tokens.get(self.offset);
        self.offset += 1;
        return tok;
    }

    fn pop(&mut self) -> shared_ptr<Token> {
        let tok = self.pop_noppdirective();
        if tok._bor().is_at_bol() && tok._bor().is(T::T_SHARP) {
            if tok._bor().has_newline_after() {
                // #
                tok._bormut().tp = T::HASH_NEWLINE;
                return tok;
            }
            let next = self.pop_noppdirective();
            if next._bor().val == "define" {
                next._bormut().tp = T::HASH_DEFINE;
                return next;
            } else {
                todo!("unimplemented directive: {}", next._bor().val);
            }
        }
        return tok;
    }

    fn define(&mut self) {
        let name = self.pop();
        if !name._bor().is(T::TOKEN_IDENT) {
            panic!("expected macro name, but found: {:?}", name);
        }

        let mut repl: shared_vec<Token> = shared_vec::new();
        while !self.is_empty() {
            let t = self.pop();
            if t._bor().has_newline_after() || t._bor().is(T::TOKEN_EOF) {
                repl.push_back(t);
                break;
            }
            repl.push_back(t);
        }

        let m = Sym::new(name._bor().val.clone(), &name, repl);
        name._bor().id.as_ref().unwrap()._bormut().sym = Some(shared_ptr::new(m));
    }

    fn get(&mut self) -> shared_ptr<Token> {
        'restart:
        while !self.is_empty() {
            let t = self.pop();

            if t._bor().is(T::HASH_DEFINE) {
                self.define();
                continue;
            }
            if self.unhide(&t) {
                continue 'restart;
            }
            if !t._bor().is(T::TOKEN_IDENT) {
                return t;
            }
            if t._bor().noexpand {
                return t;
            }

            let is_macro_name = t._bor().id.as_ref().unwrap()._bor().sym.as_ref().is_some();
            if !is_macro_name {
                return t;
            }

            let is_hidden = t._bor().id.as_ref().unwrap()._bor().sym.as_ref().unwrap()._bor().is_hidden;
            if is_hidden {
                let mut noexpand = Token::new_from(&t);
                noexpand.noexpand = true;
                return shared_ptr::new(noexpand);
            }

            self.replace_simple(t._bor().id.as_ref().unwrap()._bor().sym.as_ref().unwrap(), &t);
            continue 'restart;
        }
        return shared_ptr::new(
            Token::new(T::TOKEN_EOF
                       , "".to_string()
                       , SourceLoc::default())
        );
    }

    fn unhide(&mut self, u: &shared_ptr<Token>) -> bool {
        if u._bor().is(T::T_SPEC_UNHIDE) {
            // TODO: simplify
            u._bor().id.as_ref().unwrap()._bormut().sym.as_ref().unwrap()._bormut().unhide();
            return true;
        }
        return false;
    }

    fn replace_simple(&mut self, macros: &shared_ptr<Sym>, head: &shared_ptr<Token>) {
        macros._bormut().hide();

        let res = self.paste_all(head, macros._bor().repl._borvec());
        let mut j = (res.len() - 1) as isize;
        while j >= 0 {
            let tokp = res.get(j as usize);
            self.push(tokp);
            j -= 1;
        }
    }

    fn paste_all(&mut self, head: &shared_ptr<Token>, repl: Ref<Vec<shared_ptr<Token>>>) -> shared_vec<Token> {
        let mut rv: shared_vec<Token> = shared_vec::new();

        for tok in repl.iter() {
            let mut ntok = Token::new_from(tok);
            rv.push_back(shared_ptr::new(ntok));
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

    let mut s = Scan::new(shared_vec::new_from(tokens));
    loop {
        let tok = s.get();
        if tok._bor().is(TOKEN_EOF) {
            break;
        }
        if s.is_empty() {
            break;
        }
        println!("{}", tok._bor().val)
    }

    println!("\n:ok:\n");
}






























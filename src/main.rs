#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::{fmt, io};
use std::any::Any;
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::fmt::Write;
use std::fs::File;
use std::io::Read;
use std::mem;
use std::ops::Deref;
use std::panic::panic_any;
use std::rc::Rc;
use std::thread::current;

use tok_flags::{IS_AT_BOL, LF_AFTER, WS_BEFORE};
use token::Token;

use crate::cbuf::CBuf;
use crate::tok_maps::Keywords;
use crate::token::{Ident, T};
use crate::tokenizer::Tokenizer;

mod cbuf;
mod ascii_util;
mod tok_maps;
mod tok_flags;
mod tokenizer;
mod token;
mod tok_printer;


#[derive(Debug, Clone)]
struct Parser {
    tokens: Vec<Rc<Token>>,
    offset: usize,
    size: usize,
}

impl Parser {
    fn new(input: Vec<Token>) -> Self {
        let mut tokens: Vec<Rc<Token>> = Vec::new();
        for t in input {
            tokens.push(Rc::new(t));
        }

        let size = tokens.len();

        Parser {
            tokens,
            offset: 0,
            size,
        }
    }

    #[inline]
    fn curr(&self) -> Rc<Token> {
        assert!(self.offset < self.size);
        return Rc::clone(&self.tokens.get(self.offset).unwrap());
    }


    fn is_eof(&self) -> bool {
        if self.curr().is(T::TOKEN_EOF) {
            return true;
        }
        return self.offset >= self.size;
    }

    fn move_get(&mut self) -> Rc<Token> {
        assert!(self.offset < self.size);

        let saved = Rc::clone(&self.tokens.get(self.offset).unwrap());
        self.offset += 1;

        return saved;
    }

    fn checked_move_id(&mut self, id: &Rc<RefCell<Ident>>) -> Rc<Token> {
        if self.curr().is_ident(&id) {
            return self.move_get();
        }
        let name: RefMut<Ident> = id.borrow_mut();
        panic!("expected identifier: `{}`, but found value: `{}`", &name.name, &self.curr().value);
    }

    fn checked_mode_tp(&mut self, tp: T) -> Rc<Token> {
        // TODO: remove clone() here, add to_string() impl for T::
        if self.curr().is(tp.clone()) {
            return self.move_get();
        }
        panic!("expected token-type: `{:?}`, but found value: `{}`", &tp, &self.curr().value);
    }

    fn cut_till_newline(&mut self) -> Vec<Rc<Token>> {
        let mut result: Vec<Rc<Token>> = Vec::new();
        while !self.is_eof() {
            let tok = self.move_get();
            if tok.has_newline_after() {
                result.push(tok);
                break;
            }
            result.push(tok);
        }
        return result;
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
    let mut parser = Parser::new(tokens);

    let line = parser.cut_till_newline();
    for t in &line {
        println!("{:?}", t);
    }

    let br = parser.checked_mode_tp(T::T_LEFT_BRACE);

    println!("\n:ok:\n");
}






























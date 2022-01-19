#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::thread::current;

use tok_flags::{IS_AT_BOL, LF_AFTER, WS_BEFORE};
use token::Token;

use crate::cbuf::CBuf;
use crate::token::T;

mod cbuf;
mod ascii_util;
mod tok_maps;
mod tok_flags;
mod tokenizer;
mod token;

#[derive(Debug, Clone)]
struct Parse<'a> {
    tokens: &'a Vec<Token>,
    offset: usize,
    size: usize,
    current: &'a Token,
}

impl<'a> Parse<'a> {
    fn new(tokens: &'a Vec<Token>) -> Self {
        if tokens.is_empty() {
            panic!("Empty list of tokens.");
        }
        let size = tokens.len();
        let last = tokens.get(size - 1).unwrap();
        if !last.is(T::TOKEN_EOF) {
            panic!("No EOF at the end of the list of tokens.");
        }

        Parse {
            tokens,
            offset: 0,
            size,
            current: &tokens[0],
        }
    }

    fn move_next(&mut self) -> &Token {
        let saved = self.current;

        if self.offset >= self.size {
            panic!("Index is OOB.");
        }

        self.offset += 1;
        self.current = &self.tokens[self.offset];

        return saved;
    }

    fn checked_move(&mut self, tp: T) {
        if !self.current.is(tp.clone()) {
            panic!("Expected: {:?}, but was: {:?}", tp, self.current.tp);
        }
        self.move_next();
    }

    fn get_ident(&self) {}

    fn is_eof(&self) -> bool {
        return self.current.is(T::TOKEN_EOF);
    }
}

fn main() {
    let tokenlist: Vec<Token> = tokenizer::tokenize("fn main() {\n do{}while(0); let s = \"...\";\n let c = '.';\n return 0 ;\n }\n ");

    for tok in &tokenlist {
        if tok.has_leading_ws() {
            print!(" ");
        }
        print!("{}", tok.value);
        if tok.has_newline_after() {
            println!()
        }
    }

    let mut parser = Parse::new(&tokenlist);
    while !parser.is_eof() {
        let t = parser.move_next();
        println!("{:?}", t);
    }

    println!("{}", T::TOKEN_IDENT as i32);

    println!("\n:ok:\n");
}



























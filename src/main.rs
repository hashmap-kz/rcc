#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::{fmt, io};
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Write;
use std::fs::File;
use std::io::Read;
use std::ops::Deref;
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

pub type SharedToken = Rc<RefCell<Token>>;

fn make_shared_token(from: &Token) -> SharedToken {
    return Rc::new(RefCell::new(from.clone()));
}

#[derive(Debug, Clone)]
pub struct SharedTokenTree {
    tokens: Vec<SharedToken>,
}

impl SharedTokenTree {
    pub fn new(input: &Vec<Token>) -> Self {
        let mut tokens: Vec<SharedToken> = Vec::new();
        for t in input {
            tokens.push(make_shared_token(t));
        }
        SharedTokenTree { tokens }
    }

    fn get(&self, at: usize) -> SharedToken {
        assert!(at < self.tokens.len());
        let to_copy = self.tokens.get(at);

        assert!(to_copy.is_some());
        return Rc::clone(to_copy.unwrap());
    }

    fn len(&self) -> usize {
        self.tokens.len()
    }
}

#[derive(Debug, Clone)]
struct XParser {
    tokens: SharedTokenTree,
    offset: usize,
    size: usize,
    curr: SharedToken,
    prev: Option<SharedToken>,
}

impl XParser {
    fn new(input: &Vec<Token>) -> Self {
        if input.is_empty() {
            panic!("an empty token-list.")
        }

        let tokens = SharedTokenTree::new(input);
        let curr = tokens.get(0);
        let size = tokens.len();

        XParser {
            tokens,
            offset: 0,
            size,
            curr,
            prev: None
        }
    }

    fn is_eof(&self) -> bool {
        if self.curr.borrow_mut().is(T::TOKEN_EOF) {
            return true;
        }
        return self.offset >= self.size;
    }

    fn move_get(&mut self) -> SharedToken {
        assert!(self.offset < self.size);

        let saved = self.tokens.get(self.offset);
        self.offset += 1;

        self.prev = Some(Rc::clone(&self.curr));
        self.curr = Rc::clone(&saved);

        return saved;
    }
}

enum XTree {
    Func(SharedToken, Vec<SharedToken>), // name, parameters
}



fn main() {
    let filename = "./resources/test_data/test1.txt".to_string();

    // this one we will use through the whole program
    let keywords = Keywords::new();

    // this one we will move to tokenizer, we do not need to use it by hand.
    // we do not interested with this hash-table, its purpose to make all identifiers
    // unique, that's it.
    let identifiers= tok_maps::make_id_map(&keywords);

    let mut tokenizer = Tokenizer::new_from_file(filename, identifiers);
    let tokens = tokenizer.tokenize();
    let mut parser = XParser::new(&tokens);

    while !parser.is_eof() {
        let tok = parser.move_get();
        let x = tok.borrow_mut();
        if x.is(T::TOKEN_EOF) {
            break;
        }
        if x.is(T::TOKEN_IDENT) {
            println!("is func: {}", x.is_ident(&keywords.fn_id));
        }
        println!("{:?}", x);
    }


    println!("\n:ok:\n");
}






























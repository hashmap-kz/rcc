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

use parser::Parse;
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
mod parser;

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

fn make_id_map(keywords: &Keywords) -> HashMap<String, Rc<RefCell<Ident>>> {
    let mut idmap = HashMap::new();
    idmap.insert("as".to_string(), Rc::clone(&keywords.as_id));
    idmap.insert("break".to_string(), Rc::clone(&keywords.break_id));
    idmap.insert("continue".to_string(), Rc::clone(&keywords.continue_id));
    idmap.insert("else".to_string(), Rc::clone(&keywords.else_id));
    idmap.insert("enum".to_string(), Rc::clone(&keywords.enum_id));
    idmap.insert("false".to_string(), Rc::clone(&keywords.false_id));
    idmap.insert("fn".to_string(), Rc::clone(&keywords.fn_id));
    idmap.insert("for".to_string(), Rc::clone(&keywords.for_id));
    idmap.insert("if".to_string(), Rc::clone(&keywords.if_id));
    idmap.insert("let".to_string(), Rc::clone(&keywords.let_id));
    idmap.insert("return".to_string(), Rc::clone(&keywords.return_id));
    idmap.insert("self".to_string(), Rc::clone(&keywords.self_id));
    idmap.insert("struct".to_string(), Rc::clone(&keywords.struct_id));
    idmap.insert("true".to_string(), Rc::clone(&keywords.true_id));
    idmap.insert("while".to_string(), Rc::clone(&keywords.while_id));
    return idmap;
}

fn main() {
    let filename = "./resources/test_data/test1.txt".to_string();

    let keywords = Keywords::new(); // this one we will use through the whole program
    let idmap = make_id_map(&keywords); // this one we will move to tokenizer, we do not need to use it by hand
    let mut tokenizer = Tokenizer::new_from_file(filename, &keywords, idmap);
    let tokens = tokenizer.tokenize();
    let mut parser = XParser::new(&tokens);

    while !parser.is_eof() {
        let tok = parser.move_get();
        let x = tok.borrow_mut();
        if x.is(T::TOKEN_EOF) {
            break;
        }
        if x.is(T::TOKEN_IDENT) {
            let id = x.id.as_ref().unwrap();
            let another = &keywords.fn_id;
            println!("{}", id == another);
        }
        println!("{:?}", x);
    }


    println!("\n:ok:\n");
}






























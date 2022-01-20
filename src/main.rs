#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::{fmt, io};
use std::borrow::Borrow;
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
use crate::token::T;
use crate::tokenizer::Tokenizer;

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

fn pad(level: usize) -> String {
    let mut sb = String::new();
    for _ in 0..level {
        sb.push_str("  ");
    }
    return sb;
}

fn tokenlist_to_string_loc(tokens: &Vec<Token>) -> String {
    let mut lines: Vec<Vec<Token>> = Vec::new();
    let mut line: Vec<Token> = Vec::new();
    let mut sb = String::new();
    let mut level = 0;

    for t in tokens {
        line.push(t.clone());
        if t.has_newline_after() {
            lines.push(line);
            line = Vec::new();
        }
    }
    if !line.is_empty() {
        lines.push(line);
        line = Vec::new();
    }

    for oneline in lines {
        let mut tmp = String::new();
        let mut first = true;

        for t in oneline {
            if t.is(T::TOKEN_EOF) {
                break;
            }

            if t.is(T::T_RIGHT_BRACE) {
                level -= 1;
            }

            if first {
                let line = t.loc.line;
                write!(tmp, "{:>3}|", line).unwrap();
                write!(tmp, "{}", pad(level));
                first = false;
            }

            if t.has_leading_ws() {
                tmp.push_str(" ");
            }
            let value = &*t.value.clone();
            tmp.push_str(&value);

            if t.is(T::T_LEFT_BRACE) {
                level += 1;
            }
        }
        tmp.push_str("\n");
        sb.push_str(&*tmp);
    }

    return sb;
}

fn main() {
    let filename = "./resources/test_data/test1.txt".to_string();
    let tokenlist = Tokenizer::new_from_file(filename.to_string()).tokenize();

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

    let printable = tokenlist_to_string_loc(&tokenlist);
    println!("{}", printable);

    // An amazing example of how to save and restore parse-state.
    // If we want to look-ahead, while parsing.
    // let mut parser = Parse::new(&tokenlist);
    // let mut saved = parser.clone();
    //
    // let mut tok = parser.move_next();
    // tok = parser.move_next();
    //
    // std::mem::replace(&mut parser, saved);
    // tok = parser.move_next();

    println!("\n:ok:\n");
}






























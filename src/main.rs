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

use parser::Parse;
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
mod tok_printer;
mod parser;

fn main() {
    let filename = "./resources/test_data/test1.txt".to_string();
    let tokenlist = Tokenizer::new_from_file(filename.to_string()).tokenize();

    // How to initialize and use the parser.
    let mut parser = Parse::new(&tokenlist);
    while !parser.is_eof() {
        let t = parser.move_next();
        println!("{:?}", t);

        if t.is(T::TOKEN_IDENT) {
            let x = t.id.as_ref().unwrap();
        }
    }

    // How to flush a list of tokens to stderr.
    // let printable = tok_printer::tokens_to_string_loc(&tokenlist);
    // println!("{}", printable);


    println!("\n:ok:\n");
}






























#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

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


fn main() {
    let tokenlist: Vec<Token> = tokenizer::tokenize("fn main() {\n let s = \"...\";\n let c = '.';\n return 0 ;\n }\n ");

    for tok in &tokenlist {
        if tok.has_leading_ws() {
            print!(" ");
        }
        print!("{}", tok.value);
        if tok.has_newline_after() {
            println!()
        }
    }

    for tok in tokenlist {
        println!("{:?}", tok);
    }

    println!("\n:ok:\n");
}



























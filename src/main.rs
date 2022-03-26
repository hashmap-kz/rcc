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
mod shared;


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

    for tok in tokens {
        println!("{:?}", tok);
    }

    println!("\n:ok:\n");
}






























#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

use std::borrow::Borrow;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
enum TokenType {
    TError,
    TIdent,
}


//////////////////////////////////////////////////////////////////////
// IDENTIFIER

#[derive(Debug)]
struct Ident<'a> {
    name: &'a str,
}

impl<'a> Default for Ident<'a> {
    fn default() -> Self {
        Ident {
            name: ""
        }
    }
}

impl<'a> Ident<'a> {
    fn new(name: &'a str) -> Self {
        Ident {
            name
        }
    }
}

impl<'a> PartialEq for Ident<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl<'a> Eq for Ident<'a> {}

static EMPTY_IDENT: Ident = Ident { name: "" };
static FOR_IDENT: Ident = Ident { name: "for" };

//////////////////////////////////////////////////////////////////////
// TOKEN

#[derive(Debug)]
struct Token<'a> {
    tp: TokenType,
    value: Rc<String>,
    ident: &'a Ident<'a>,
}

impl<'a> Default for Token<'a> {
    fn default() -> Self {
        Token {
            tp: TokenType::TError,
            value: Rc::new("".to_string()),
            ident: &EMPTY_IDENT,
        }
    }
}

impl<'a> Token<'a> {
    fn new(tp: TokenType, value: Rc<String>) -> Token<'a> {
        Token {
            tp,
            value,
            ident: &EMPTY_IDENT,
        }
    }

    fn new_ident(ident: &'a Ident) -> Token<'a> {
        Token {
            tp: TokenType::TIdent,
            value: Rc::new(ident.name.parse().unwrap()),
            ident,
        }
    }

    fn is(&self, tp: TokenType) -> bool {
        self.tp == tp
    }

    fn is_ident(&self, ident: &Ident) -> bool {
        if !self.is(TokenType::TIdent) {
            return false;
        }
        return self.ident == ident;
    }
}

fn main() {
    let x = Ident::new("main");

    let mut buffer = String::from("0x");
    buffer.push_str("ff");

    let another_id = Ident::new(buffer.as_str());

    let tok1 = Token::new_ident(&FOR_IDENT);
    let tok2 = Token::new_ident(&x);
    let tok3 = Token::new_ident(&another_id);

    let tokenlist = vec![tok1, tok2, tok3];

    for tok in tokenlist {
        println!("{:?}", tok);
    }

    println!("\n:ok:\n");
}


//////////////////////////////////////////////////////////////////////
// TESTS

#[cfg(test)]
mod tests {
    use crate::{FOR_IDENT, Ident};

    #[test]
    fn test_idents_are_eq() {
        let x = Ident::new("for");
        assert_eq!(x, FOR_IDENT);
    }
}
































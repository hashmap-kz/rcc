use std::fmt;
use std::rc::Rc;

use crate::tok_flags::{IS_AT_BOL, LF_AFTER, WS_BEFORE};

#[derive(Debug, PartialEq, Eq, Clone)]
#[allow(non_camel_case_types)]
pub enum T {
    TOKEN_ERROR,
    TOKEN_EOF,

    // These two are temporary. They won't be present in the result list of tokens.
    TOKEN_WS,
    TOKEN_LF,

    TOKEN_IDENT(Rc<String>),
    TOKEN_NUMBER,
    TOKEN_CHAR,
    TOKEN_STRING,
    TOKEN_COMMENT,

    T_ARROW,
    T_MINUS_MINUS,
    T_MINUS_EQUAL,
    T_NE,
    T_DOT_DOT,
    T_TIMES_EQUAL,
    T_DIVIDE_EQUAL,
    T_AND_EQUAL,
    T_AND_AND,
    T_SHARP_SHARP,
    T_PERCENT_EQUAL,
    T_XOR_EQUAL,
    T_PLUS_PLUS,
    T_PLUS_EQUAL,
    T_LE,
    T_LSHIFT,
    T_EQ,
    T_GE,
    T_RSHIFT,
    T_OR_OR,
    T_OR_EQUAL,
    T_COLON_COLON,
    T_DOT_DOT_DOT,
    T_LSHIFT_EQUAL,
    T_RSHIFT_EQUAL,
    T_COMMA,
    T_MINUS,
    T_SEMI_COLON,
    T_COLON,
    T_EXCLAMATION,
    T_QUESTION,
    T_DOT,
    T_LEFT_PAREN,
    T_RIGHT_PAREN,
    T_LEFT_BRACKET,
    T_RIGHT_BRACKET,
    T_LEFT_BRACE,
    T_RIGHT_BRACE,
    T_TIMES,
    T_DIVIDE,
    T_AND,
    T_SHARP,
    T_PERCENT,
    T_XOR,
    T_PLUS,
    T_LT,
    T_ASSIGN,
    T_GT,
    T_OR,
    T_TILDE,

    T_DOLLAR_SIGN,
    T_AT_SIGN,
    T_GRAVE_ACCENT,
    T_BACKSLASH,
}

#[derive(PartialEq, Eq, Clone)]
pub struct Token {
    pub(crate) tp: T,
    pub(crate) value: Rc<String>,
    pub(crate) pos: i32,
}

impl<'a> Default for Token {
    fn default() -> Self {
        Token {
            tp: T::TOKEN_ERROR,
            value: Rc::new("".to_string()),
            pos: 0,
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut flag = String::new();
        if self.has_leading_ws() {
            flag.push_str("WS");
        }
        if self.has_newline_after() {
            if !flag.is_empty() {
                flag.push_str("|");
            }
            flag.push_str("LF");
        }
        if self.is_at_bol() {
            if !flag.is_empty() {
                flag.push_str("|");
            }
            flag.push_str("BOL");
        }

        f.debug_struct("Token")
            .field("tp", &self.tp)
            .field("value", &self.value)
            .field("pos", &flag)
            .finish()
    }
}

impl<'a> Token {
    fn new(tp: T, value: &'a str) -> Self {
        Token {
            tp,
            value: Rc::new(value.to_string()),
            pos: 0,
        }
    }

    pub(crate) fn new_ident(ident: Rc<String>) -> Self {
        Token {
            tp: T::TOKEN_IDENT(ident.clone()),
            value: ident,
            pos: 0,
        }
    }


    pub(crate) fn make_eof() -> Self {
        Token { tp: T::TOKEN_EOF, value: Rc::new("".to_string()), pos: 0 }
    }

    pub(crate) fn make_ws() -> Self {
        Token { tp: T::TOKEN_WS, value: Rc::new(" ".to_string()), pos: 0 }
    }

    pub(crate) fn make_lf() -> Self {
        Token { tp: T::TOKEN_LF, value: Rc::new("\n".to_string()), pos: 0 }
    }

    pub(crate) fn is(&self, tp: T) -> bool {
        self.tp == tp
    }

    pub(crate) fn is_ident(&self, ident: &'a str) -> bool {
        todo!()
    }

    pub(crate) fn has_leading_ws(&self) -> bool {
        return (self.pos & WS_BEFORE) == WS_BEFORE;
    }

    pub(crate) fn has_newline_after(&self) -> bool {
        return (self.pos & LF_AFTER) == LF_AFTER;
    }

    pub(crate) fn is_at_bol(&self) -> bool {
        return (self.pos & IS_AT_BOL) == IS_AT_BOL;
    }
}

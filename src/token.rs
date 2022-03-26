use std::cell::RefCell;
use std::fmt;
use std::fmt::Write;
use std::rc::Rc;

use crate::shared::shared_ptr;
use crate::tok_flags::{IS_AT_BOL, LF_AFTER, WS_BEFORE};
use crate::token::T::TOKEN_IDENT;

//@formatter:off
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone)]
#[allow(non_camel_case_types)]
pub enum T {
    TOKEN_ERROR,
    TOKEN_EOF,

    // These two are temporary.
    // They won't be present in the result list of tokens.
    TOKEN_WS,
    TOKEN_LF,

    // General identifier, may be a user-defined-name, or a keyword.
    TOKEN_IDENT,
    TOKEN_NUMBER,
    TOKEN_CHAR,
    TOKEN_STRING,
    TOKEN_COMMENT,

    T_RSHIFT_EQUAL,
    // >>=
    T_LSHIFT_EQUAL,
    // <<=
    T_DOT_DOT_DOT,
    // ...
    T_ARROW,
    // ->
    T_MINUS_MINUS,
    // --
    T_MINUS_EQUAL,
    // -=
    T_NE,
    // !=
    T_DOT_DOT,
    // ..
    T_TIMES_EQUAL,
    // *=
    T_DIVIDE_EQUAL,
    // /=
    T_AND_EQUAL,
    // &=
    T_AND_AND,
    // &&
    T_SHARP_SHARP,
    // ##
    T_PERCENT_EQUAL,
    // %=
    T_XOR_EQUAL,
    // ^=
    T_PLUS_PLUS,
    // ++
    T_PLUS_EQUAL,
    // +=
    T_LE,
    // <=
    T_LSHIFT,
    // <<
    T_EQ,
    // ==
    T_GE,
    // >=
    T_RSHIFT,
    // >>
    T_OR_OR,
    // ||
    T_OR_EQUAL,
    // |=
    T_COMMA,
    // ,
    T_MINUS,
    // -
    T_SEMI_COLON,
    // ;
    T_COLON,
    // :
    T_EXCLAMATION,
    // !
    T_QUESTION,
    // ?
    T_DOT,
    // .
    T_LEFT_PAREN,
    // (
    T_RIGHT_PAREN,
    // )
    T_LEFT_BRACKET,
    // [
    T_RIGHT_BRACKET,
    // ]
    T_LEFT_BRACE,
    // {
    T_RIGHT_BRACE,
    // }
    T_TIMES,
    // *
    T_DIVIDE,
    // /
    T_AND,
    // &
    T_SHARP,
    // #
    T_PERCENT,
    // %
    T_XOR,
    // ^
    T_PLUS,
    // +
    T_LT,
    // <
    T_ASSIGN,
    // =
    T_GT,
    // >
    T_OR,
    // |
    T_TILDE,
    // ~
    T_DOLLAR_SIGN,
    // $
    T_AT_SIGN,
    // @
    T_GRAVE_ACCENT,
    // `
    T_BACKSLASH, // \

    // preprocessor
    T_SPEC_UNHIDE,
    T_SPEC_PLACEMARKER,
    HASH_NEWLINE,
    HASH_DEFINE,
    HASH_INCLUDE,
}
//@formatter:on

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SourceLoc {
    pub(crate) file: Rc<String>,
    pub(crate) line: i32,
    pub(crate) column: i32,
}

impl Default for SourceLoc {
    fn default() -> Self {
        SourceLoc {
            file: Rc::new("".to_string()),
            line: 0,
            column: 0,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Ident {
    pub name: String,
}

impl Ident {
    pub fn new(name: String) -> Self {
        Ident {
            name,
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Token {
    pub tp: T,
    pub value: String,
    pub pos: u32,
    pub cat: u32,
    pub loc: SourceLoc,
    pub id: Option<shared_ptr<Ident>>,
    pub noexpand: bool,
}

impl Default for Token {
    fn default() -> Self {
        Token {
            tp: T::TOKEN_ERROR,
            value: "".to_string(),
            pos: 0,
            cat: 0,
            loc: SourceLoc::default(),
            id: None,
            noexpand: false,
        }
    }
}

impl SourceLoc {
    pub fn new(file: Rc<String>, line: i32, column: i32) -> Self {
        SourceLoc {
            file,
            line,
            column,
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

        // TODO: normal column offset from value length.
        let loc = format!("{}:{}:{}"
                          , &self.loc.file
                          , &self.loc.line
                          , &self.loc.column);

        let mut ident = String::from("");
        if self.is(T::TOKEN_IDENT) {
            let x = self.id.as_ref().unwrap()._bor();
            ident = String::from("");
            write!(ident, "{}", x.name.clone()).unwrap();
        }

        f.debug_struct("Token")
            .field("tp", &self.tp)
            .field("value", &self.value)
            .field("pos", &flag)
            .field("loc", &loc)
            .field("id", &ident)
            .finish()
    }
}

impl Token {
    pub(crate) fn new(tp: T, value: String, loc: SourceLoc) -> Self {
        Token {
            tp,
            value,
            pos: 0,
            cat: 0,
            loc,
            id: None,
            noexpand: false,
        }
    }

    pub(crate) fn new_from(another: Rc<Token>) -> Self {
        Token {
            tp: another.tp.clone(),
            value: another.value.clone(),
            pos: another.pos,
            cat: another.cat,
            loc: another.loc.clone(),
            id: another.id.clone(),
            noexpand: another.noexpand,
        }
    }


    pub(crate) fn make_eof() -> Self {
        Token { tp: T::TOKEN_EOF, value: "".to_string(), pos: 0, cat: 0, loc: SourceLoc::default(), id: None, noexpand: false }
    }

    pub(crate) fn make_ws() -> Self {
        Token { tp: T::TOKEN_WS, value: " ".to_string(), pos: 0, cat: 0, loc: SourceLoc::default(), id: None, noexpand: false }
    }

    pub(crate) fn make_lf() -> Self {
        Token { tp: T::TOKEN_LF, value: "\n".to_string(), pos: 0, cat: 0, loc: SourceLoc::default(), id: None, noexpand: false }
    }

    pub(crate) fn is(&self, tp: T) -> bool {
        self.tp == tp
    }

    pub fn is_ident(&self, what: &shared_ptr<Ident>) -> bool {
        if !self.is(T::TOKEN_IDENT) {
            return false;
        }
        return self.id.as_ref().unwrap() == what;
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

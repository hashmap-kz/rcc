use std::cell::{Ref, RefCell, RefMut};
use std::fmt;
use std::fmt::Write;
use std::rc::Rc;

use crate::shared::{shared_ptr, shared_vec};
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
    T_LSHIFT_EQUAL,
    T_DOT_DOT_DOT,
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

    T_SPEC_UNHIDE,
    HASH_NEWLINE,
    HASH_DEFINE,
    HASH_INCLUDE,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SourceLoc {
    pub file: Rc<String>,
    pub line: i32,
    pub column: i32,
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
pub struct Sym {
    pub repl: shared_vec<Token>,
    pub is_hidden: bool,
}

impl Sym {
    pub fn new(name: String, head: &shared_ptr<Token>, mut repl: shared_vec<Token>) -> Self {
        let mut unhide = Token::new_from(head);
        unhide.tp = T::T_SPEC_UNHIDE;
        repl.push_back(shared_ptr::new(unhide));

        Sym {
            repl,
            is_hidden: false,
        }
    }

    pub fn hide(&mut self) {
        assert!(!self.is_hidden);
        self.is_hidden = true;
    }

    pub fn unhide(&mut self) {
        assert!(self.is_hidden);
        self.is_hidden = false;
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Ident {
    pub name: String,
    pub sym: Option<shared_ptr<Sym>>,
}

impl Ident {
    pub fn new(name: String) -> Self {
        Ident {
            name,
            sym: None,
        }
    }

    #[inline]
    pub fn set_sym(&mut self, sym: shared_ptr<Sym>) {
        self.sym = Some(sym);
    }

    #[inline]
    pub fn get_mut_sym(&self) -> RefMut<Sym> {
        // TODO: check
        return self.sym.as_ref().unwrap()._bormut();
    }

    #[inline]
    pub fn get_nomut_sym(&self) -> RefMut<Sym> {
        // TODO: check
        return self.sym.as_ref().unwrap()._bormut();
    }

    #[inline]
    pub fn has_sym(&self) -> bool {
        return self.sym.is_some();
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Token {
    pub tp: T,
    pub val: String,
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
            val: "".to_string(),
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
            .field("value", &self.val)
            .field("pos", &flag)
            .field("loc", &loc)
            .field("id", &ident)
            .finish()
    }
}

impl Token {
    pub fn new(tp: T, value: String, loc: SourceLoc) -> Self {
        Token {
            tp,
            val: value,
            pos: 0,
            cat: 0,
            loc,
            id: None,
            noexpand: false,
        }
    }

    pub fn new_from(another: &shared_ptr<Token>) -> Self {
        let to_copy = another._bor();
        let value = String::from(to_copy.val.clone());
        let id = to_copy.copy_ident();
        Token {
            tp: to_copy.tp.clone(),
            val: value,
            pos: to_copy.pos,
            cat: to_copy.cat,
            loc: to_copy.loc.clone(),
            id,
            noexpand: to_copy.noexpand,
        }
    }

    fn copy_ident(&self) -> Option<shared_ptr<Ident>> {
        let mut id: Option<shared_ptr<Ident>> = None;
        if self.id.is_some() {
            id = Option::from(shared_ptr::_cloneref(self.id.as_ref().unwrap()));
        }
        return id;
    }

    pub fn make_eof() -> Self {
        Token { tp: T::TOKEN_EOF, val: "".to_string(), pos: 0, cat: 0, loc: SourceLoc::default(), id: None, noexpand: false }
    }

    pub fn make_ws() -> Self {
        Token { tp: T::TOKEN_WS, val: " ".to_string(), pos: 0, cat: 0, loc: SourceLoc::default(), id: None, noexpand: false }
    }

    pub fn make_lf() -> Self {
        Token { tp: T::TOKEN_LF, val: "\n".to_string(), pos: 0, cat: 0, loc: SourceLoc::default(), id: None, noexpand: false }
    }

    #[inline]
    pub fn is(&self, tp: T) -> bool {
        self.tp == tp
    }

    pub fn is_ident(&self, what: &shared_ptr<Ident>) -> bool {
        if !self.is(T::TOKEN_IDENT) {
            return false;
        }
        return self.id.as_ref().unwrap() == what;
    }

    #[inline]
    pub fn get_mut_ident(&self) -> RefMut<Ident> {
        // TODO: check
        return self.id.as_ref().unwrap()._bormut();
    }

    #[inline]
    pub fn get_nomut_ident(&self) -> Ref<Ident> {
        // TODO: check
        return self.id.as_ref().unwrap()._bor();
    }

    #[inline]
    pub fn is_macro_name(&self) -> bool {
        if !self.is(T::TOKEN_IDENT) {
            return false;
        }
        return self.get_nomut_ident().has_sym();
    }

    pub fn has_leading_ws(&self) -> bool {
        return (self.pos & WS_BEFORE) == WS_BEFORE;
    }

    pub fn has_newline_after(&self) -> bool {
        return (self.pos & LF_AFTER) == LF_AFTER;
    }

    pub fn is_at_bol(&self) -> bool {
        return (self.pos & IS_AT_BOL) == IS_AT_BOL;
    }
}

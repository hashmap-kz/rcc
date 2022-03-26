use std::cell::{Ref, RefCell, RefMut};
use std::fmt;
use std::fmt::Write;
use std::rc::Rc;

use crate::ident::Ident;
use crate::shared::{shared_ptr, shared_vec};
use crate::sloc::SourceLoc;
use crate::tok_flags::{IS_AT_BOL, LF_AFTER, WS_BEFORE};
use crate::toktype::T;
use crate::toktype::T::TOKEN_IDENT;

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

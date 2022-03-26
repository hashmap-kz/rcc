use crate::{shared_ptr, shared_vec, Token};
use crate::toktype::T;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Sym {
    pub repl: shared_vec<Token>,
    pub is_hidden: bool,
}

impl Sym {
    pub fn new(head: &shared_ptr<Token>, mut repl: shared_vec<Token>) -> Self {
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

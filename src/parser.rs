use crate::token::{T, Token};

#[derive(Debug, Clone)]
pub struct Parse<'a> {
    tokens: &'a Vec<Token>,
    offset: usize,
    size: usize,
    current: &'a Token,
}

impl<'a> Parse<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
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

    pub fn move_next(&mut self) -> &Token {
        let saved = self.current;

        if self.offset >= self.size {
            panic!("Index is OOB.");
        }

        self.offset += 1;
        self.current = &self.tokens[self.offset];

        return saved;
    }

    pub fn checked_move(&mut self, tp: T) {
        if !self.current.is(tp.clone()) {
            panic!("Expected: {:?}, but was: {:?}", tp, self.current.tp);
        }
        self.move_next();
    }

    pub fn get_ident(&self) {}

    pub fn is_eof(&self) -> bool {
        return self.current.is(T::TOKEN_EOF);
    }
}

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
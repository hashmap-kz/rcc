use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::token::{Ident, T};

#[derive(Debug, Eq, PartialEq)]
pub struct Keywords {
    pub as_id: Rc<RefCell<Ident>>,
    pub break_id: Rc<RefCell<Ident>>,
    pub continue_id: Rc<RefCell<Ident>>,
    pub else_id: Rc<RefCell<Ident>>,
    pub enum_id: Rc<RefCell<Ident>>,
    pub false_id: Rc<RefCell<Ident>>,
    pub fn_id: Rc<RefCell<Ident>>,
    pub for_id: Rc<RefCell<Ident>>,
    pub if_id: Rc<RefCell<Ident>>,
    pub let_id: Rc<RefCell<Ident>>,
    pub return_id: Rc<RefCell<Ident>>,
    pub self_id: Rc<RefCell<Ident>>,
    pub struct_id: Rc<RefCell<Ident>>,
    pub true_id: Rc<RefCell<Ident>>,
    pub while_id: Rc<RefCell<Ident>>,
}

impl Keywords {
    pub fn new() -> Self {
        Keywords {
            as_id: Rc::new(RefCell::new(Ident::new("as".to_string(), 0))),
            break_id: Rc::new(RefCell::new(Ident::new("break".to_string(), 1))),
            continue_id: Rc::new(RefCell::new(Ident::new("continue".to_string(), 2))),
            else_id: Rc::new(RefCell::new(Ident::new("else".to_string(), 3))),
            enum_id: Rc::new(RefCell::new(Ident::new("enum".to_string(), 4))),
            false_id: Rc::new(RefCell::new(Ident::new("false".to_string(), 5))),
            fn_id: Rc::new(RefCell::new(Ident::new("fn".to_string(), 6))),
            for_id: Rc::new(RefCell::new(Ident::new("for".to_string(), 7))),
            if_id: Rc::new(RefCell::new(Ident::new("if".to_string(), 8))),
            let_id: Rc::new(RefCell::new(Ident::new("let".to_string(), 9))),
            return_id: Rc::new(RefCell::new(Ident::new("return".to_string(), 10))),
            self_id: Rc::new(RefCell::new(Ident::new("self".to_string(), 11))),
            struct_id: Rc::new(RefCell::new(Ident::new("struct".to_string(), 12))),
            true_id: Rc::new(RefCell::new(Ident::new("true".to_string(), 13))),
            while_id: Rc::new(RefCell::new(Ident::new("while".to_string(), 14))),
        }
    }
}

pub fn make_id_map(keywords: &Keywords) -> HashMap<String, Rc<RefCell<Ident>>> {
    let mut idmap = HashMap::new();
    idmap.insert("as".to_string(), Rc::clone(&keywords.as_id));
    idmap.insert("break".to_string(), Rc::clone(&keywords.break_id));
    idmap.insert("continue".to_string(), Rc::clone(&keywords.continue_id));
    idmap.insert("else".to_string(), Rc::clone(&keywords.else_id));
    idmap.insert("enum".to_string(), Rc::clone(&keywords.enum_id));
    idmap.insert("false".to_string(), Rc::clone(&keywords.false_id));
    idmap.insert("fn".to_string(), Rc::clone(&keywords.fn_id));
    idmap.insert("for".to_string(), Rc::clone(&keywords.for_id));
    idmap.insert("if".to_string(), Rc::clone(&keywords.if_id));
    idmap.insert("let".to_string(), Rc::clone(&keywords.let_id));
    idmap.insert("return".to_string(), Rc::clone(&keywords.return_id));
    idmap.insert("self".to_string(), Rc::clone(&keywords.self_id));
    idmap.insert("struct".to_string(), Rc::clone(&keywords.struct_id));
    idmap.insert("true".to_string(), Rc::clone(&keywords.true_id));
    idmap.insert("while".to_string(), Rc::clone(&keywords.while_id));
    return idmap;
}

pub fn make_maps() -> HashMap<&'static str, T> {
    let mut punct_map: HashMap<&str, T> = HashMap::new();

    // " ... && -= >= ~ + ; ] <: "
    // " <<= &= -> >> % , < ^ :> "
    // " >>= *= /= ^= & - = { <% "
    // " != ++ << |= ( . > | %> "
    // " %= += <= || ) / ? } %: "
    // " ## -- == ! * : [ # %:%: "

    // c-digraphs
    punct_map.insert("%:%:", T::T_SHARP_SHARP);
    punct_map.insert("<:", T::T_LEFT_BRACKET);
    punct_map.insert(":>", T::T_RIGHT_BRACKET);
    punct_map.insert("<%", T::T_LEFT_BRACE);
    punct_map.insert("%>", T::T_RIGHT_BRACE);
    punct_map.insert("%:", T::T_SHARP);

    punct_map.insert(">>=", T::T_RSHIFT_EQUAL);
    punct_map.insert("<<=", T::T_LSHIFT_EQUAL);
    punct_map.insert("...", T::T_DOT_DOT_DOT);
    punct_map.insert("->", T::T_ARROW);
    punct_map.insert("--", T::T_MINUS_MINUS);
    punct_map.insert("-=", T::T_MINUS_EQUAL);
    punct_map.insert("!=", T::T_NE);
    punct_map.insert("..", T::T_DOT_DOT);
    punct_map.insert("*=", T::T_TIMES_EQUAL);
    punct_map.insert("/=", T::T_DIVIDE_EQUAL);
    punct_map.insert("&=", T::T_AND_EQUAL);
    punct_map.insert("&&", T::T_AND_AND);
    punct_map.insert("##", T::T_SHARP_SHARP);
    punct_map.insert("%=", T::T_PERCENT_EQUAL);
    punct_map.insert("^=", T::T_XOR_EQUAL);
    punct_map.insert("++", T::T_PLUS_PLUS);
    punct_map.insert("+=", T::T_PLUS_EQUAL);
    punct_map.insert("<=", T::T_LE);
    punct_map.insert("<<", T::T_LSHIFT);
    punct_map.insert("==", T::T_EQ);
    punct_map.insert(">=", T::T_GE);
    punct_map.insert(">>", T::T_RSHIFT);
    punct_map.insert("||", T::T_OR_OR);
    punct_map.insert("|=", T::T_OR_EQUAL);
    punct_map.insert(",", T::T_COMMA);
    punct_map.insert("-", T::T_MINUS);
    punct_map.insert(";", T::T_SEMI_COLON);
    punct_map.insert(":", T::T_COLON);
    punct_map.insert("!", T::T_EXCLAMATION);
    punct_map.insert("?", T::T_QUESTION);
    punct_map.insert(".", T::T_DOT);
    punct_map.insert("(", T::T_LEFT_PAREN);
    punct_map.insert(")", T::T_RIGHT_PAREN);
    punct_map.insert("[", T::T_LEFT_BRACKET);
    punct_map.insert("]", T::T_RIGHT_BRACKET);
    punct_map.insert("{", T::T_LEFT_BRACE);
    punct_map.insert("}", T::T_RIGHT_BRACE);
    punct_map.insert("*", T::T_TIMES);
    punct_map.insert("/", T::T_DIVIDE);
    punct_map.insert("&", T::T_AND);
    punct_map.insert("#", T::T_SHARP);
    punct_map.insert("%", T::T_PERCENT);
    punct_map.insert("^", T::T_XOR);
    punct_map.insert("+", T::T_PLUS);
    punct_map.insert("<", T::T_LT);
    punct_map.insert("=", T::T_ASSIGN);
    punct_map.insert(">", T::T_GT);
    punct_map.insert("|", T::T_OR);
    punct_map.insert("~", T::T_TILDE);

    // not operators, other ascii characters we have to recognize
    punct_map.insert("$", T::T_DOLLAR_SIGN);
    punct_map.insert("@", T::T_AT_SIGN);
    punct_map.insert("`", T::T_GRAVE_ACCENT);
    punct_map.insert("\\", T::T_BACKSLASH);

    return punct_map;
}

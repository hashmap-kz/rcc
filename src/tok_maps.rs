use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::shared::shared_ptr;

use crate::token::{Ident, T};

#[derive(Debug, Eq, PartialEq)]
#[allow(non_snake_case)]
pub struct Keywords {
    pub auto_ident: shared_ptr<Ident>,
    pub break_ident: shared_ptr<Ident>,
}

impl Keywords {
    pub fn new() -> Self {
        Keywords {
            auto_ident: shared_ptr::new(Ident::new("auto".to_string())),
            break_ident: shared_ptr::new(Ident::new("break".to_string())),
        }
    }
}

pub fn make_id_map(keywords: &Keywords) -> HashMap<String, shared_ptr<Ident>> {
    let mut idmap = HashMap::new();

    idmap.insert("auto".to_string(), shared_ptr::_cloneref(&keywords.auto_ident));
    idmap.insert("break".to_string(), shared_ptr::_cloneref(&keywords.break_ident));

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

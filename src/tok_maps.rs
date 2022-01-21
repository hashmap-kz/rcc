use std::collections::HashMap;

use crate::token::T;

pub fn make_maps() -> (HashMap<&'static str, T>, HashMap<&'static str, T>, HashMap<&'static str, T>, HashMap<&'static str, T>, HashMap<&'static str, T>) {
    let mut punct_map_3: HashMap<&str, T> = HashMap::new();
    let mut punct_map_2: HashMap<&str, T> = HashMap::new();
    let mut punct_map_1: HashMap<&str, T> = HashMap::new();
    let mut punct_map_u: HashMap<&str, T> = HashMap::new();
    let mut keywords: HashMap<&str, T> = HashMap::new();

    // " ... && -= >= ~ + ; ] <: "
    // " <<= &= -> >> % , < ^ :> "
    // " >>= *= /= ^= & - = { <% "
    // " != ++ << |= ( . > | %> "
    // " %= += <= || ) / ? } %: "
    // " ## -- == ! * : [ # %:%: "

    // 3
    punct_map_3.insert(">>=", T::T_RSHIFT_EQUAL);
    punct_map_3.insert("<<=", T::T_LSHIFT_EQUAL);
    punct_map_3.insert("...", T::T_DOT_DOT_DOT);
    punct_map_3.insert("..=", T::T_DOT_DOT_EQ);

    // 2
    punct_map_2.insert("->", T::T_ARROW);
    punct_map_2.insert("=>", T::T_ARROW2);
    punct_map_2.insert("--", T::T_MINUS_MINUS);
    punct_map_2.insert("-=", T::T_MINUS_EQUAL);
    punct_map_2.insert("!=", T::T_NE);
    punct_map_2.insert("..", T::T_DOT_DOT);
    punct_map_2.insert("*=", T::T_TIMES_EQUAL);
    punct_map_2.insert("/=", T::T_DIVIDE_EQUAL);
    punct_map_2.insert("&=", T::T_AND_EQUAL);
    punct_map_2.insert("&&", T::T_AND_AND);
    punct_map_2.insert("##", T::T_SHARP_SHARP);
    punct_map_2.insert("%=", T::T_PERCENT_EQUAL);
    punct_map_2.insert("^=", T::T_XOR_EQUAL);
    punct_map_2.insert("++", T::T_PLUS_PLUS);
    punct_map_2.insert("+=", T::T_PLUS_EQUAL);
    punct_map_2.insert("<=", T::T_LE);
    punct_map_2.insert("<<", T::T_LSHIFT);
    punct_map_2.insert("==", T::T_EQ);
    punct_map_2.insert(">=", T::T_GE);

    // we'll handle this '>>' as a special case in
    // expression parsing stage,
    // because it's much easier to handle this situation: list<list<i32>>
    // in expression, instead of to rewrite the whole parser logic...
    // ::
    // punct_map_2.insert(">>", T.T_RSHIFT);

    punct_map_2.insert("||", T::T_OR_OR);
    punct_map_2.insert("|=", T::T_OR_EQUAL);
    punct_map_2.insert("::", T::T_COLON_COLON);

    // 1
    punct_map_1.insert(",", T::T_COMMA);
    punct_map_1.insert("-", T::T_MINUS);
    punct_map_1.insert(";", T::T_SEMI_COLON);
    punct_map_1.insert(":", T::T_COLON);
    punct_map_1.insert("!", T::T_EXCLAMATION);
    punct_map_1.insert("?", T::T_QUESTION);
    punct_map_1.insert(".", T::T_DOT);
    punct_map_1.insert("(", T::T_LEFT_PAREN);
    punct_map_1.insert(")", T::T_RIGHT_PAREN);
    punct_map_1.insert("[", T::T_LEFT_BRACKET);
    punct_map_1.insert("]", T::T_RIGHT_BRACKET);
    punct_map_1.insert("{", T::T_LEFT_BRACE);
    punct_map_1.insert("}", T::T_RIGHT_BRACE);
    punct_map_1.insert("*", T::T_TIMES);
    punct_map_1.insert("/", T::T_DIVIDE);
    punct_map_1.insert("&", T::T_AND);
    punct_map_1.insert("#", T::T_SHARP);
    punct_map_1.insert("%", T::T_PERCENT);
    punct_map_1.insert("^", T::T_XOR);
    punct_map_1.insert("+", T::T_PLUS);
    punct_map_1.insert("<", T::T_LT);
    punct_map_1.insert("=", T::T_ASSIGN);
    punct_map_1.insert(">", T::T_GT);
    punct_map_1.insert("|", T::T_OR);
    punct_map_1.insert("~", T::T_TILDE);

    // unspecified by language, but still usable somehow
    punct_map_u.insert("$", T::T_DOLLAR_SIGN);
    punct_map_u.insert("@", T::T_AT_SIGN);
    punct_map_u.insert("`", T::T_GRAVE_ACCENT);
    punct_map_u.insert("\\", T::T_BACKSLASH);

    // // Keywords
    // keywords.insert("break", T::break_ident);
    // keywords.insert("continue", T::continue_ident);
    // keywords.insert("do", T::do_ident);
    // keywords.insert("else", T::else_ident);
    // keywords.insert("for", T::for_ident);
    // keywords.insert("if", T::if_ident);
    // keywords.insert("return", T::return_ident);
    // keywords.insert("while", T::while_ident);
    // keywords.insert("static", T::static_ident);
    // keywords.insert("pub", T::pub_ident);
    // keywords.insert("true", T::true_ident);
    // keywords.insert("false", T::false_ident);
    // keywords.insert("self", T::self_ident);
    // keywords.insert("default", T::default_ident);
    // keywords.insert("static_assert", T::static_assert_ident);
    // keywords.insert("assert_true", T::assert_true_ident);
    // keywords.insert("char", T::char_ident);
    // keywords.insert("u8", T::u8_ident);
    // keywords.insert("i32", T::i32_ident);
    // keywords.insert("bool", T::bool_ident);
    // keywords.insert("struct", T::struct_ident);


    return (punct_map_3, punct_map_2, punct_map_1, punct_map_u, keywords);
}

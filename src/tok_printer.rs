use crate::token::{T, Token};
use std::fmt::Write;

fn pad(level: usize) -> String {
    let mut sb = String::new();
    for _ in 0..level {
        sb.push_str("  ");
    }
    return sb;
}

pub fn tokens_to_string_loc(tokens: &Vec<Token>) -> String {
    let mut lines: Vec<Vec<Token>> = Vec::new();
    let mut line: Vec<Token> = Vec::new();
    let mut sb = String::new();
    let mut level = 0;

    for t in tokens {
        line.push(t.clone());
        if t.has_newline_after() {
            lines.push(line);
            line = Vec::new();
        }
    }
    if !line.is_empty() {
        lines.push(line);
    }

    for oneline in lines {
        let mut tmp = String::new();
        let mut first = true;

        for t in oneline {
            if t.is(T::TOKEN_EOF) {
                break;
            }

            if t.is(T::T_RIGHT_BRACE) {
                level -= 1;
            }

            if first {
                let line = t.loc.line;
                write!(tmp, "{:>3}|", line).unwrap();
                write!(tmp, "{}", pad(level)).unwrap();
                first = false;
            }

            if t.has_leading_ws() {
                tmp.push_str(" ");
            }
            let value = &*t.value.clone();
            tmp.push_str(&value);

            if t.is(T::T_LEFT_BRACE) {
                level += 1;
            }
        }
        tmp.push_str("\n");
        sb.push_str(&*tmp);
    }

    return sb;
}

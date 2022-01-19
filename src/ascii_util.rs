pub fn is_letter(c: u8) -> bool {
    return c == b'a' ||
        c == b'b' ||
        c == b'c' ||
        c == b'd' ||
        c == b'e' ||
        c == b'f' ||
        c == b'g' ||
        c == b'h' ||
        c == b'i' ||
        c == b'j' ||
        c == b'k' ||
        c == b'l' ||
        c == b'm' ||
        c == b'n' ||
        c == b'o' ||
        c == b'p' ||
        c == b'q' ||
        c == b'r' ||
        c == b's' ||
        c == b't' ||
        c == b'u' ||
        c == b'v' ||
        c == b'w' ||
        c == b'x' ||
        c == b'y' ||
        c == b'z' ||
        c == b'A' ||
        c == b'B' ||
        c == b'C' ||
        c == b'D' ||
        c == b'E' ||
        c == b'F' ||
        c == b'G' ||
        c == b'H' ||
        c == b'I' ||
        c == b'J' ||
        c == b'K' ||
        c == b'L' ||
        c == b'M' ||
        c == b'N' ||
        c == b'O' ||
        c == b'P' ||
        c == b'Q' ||
        c == b'R' ||
        c == b'S' ||
        c == b'T' ||
        c == b'U' ||
        c == b'V' ||
        c == b'W' ||
        c == b'X' ||
        c == b'Y' ||
        c == b'Z' ||
        c == b'_';
}

pub fn is_dec(c: u8) -> bool {
    return c == b'0' ||
        c == b'1' ||
        c == b'2' ||
        c == b'3' ||
        c == b'4' ||
        c == b'5' ||
        c == b'6' ||
        c == b'7' ||
        c == b'8' ||
        c == b'9';
}

pub fn is_hex(c: u8) -> bool {
    return is_dec(c) ||
        c == b'A' ||
        c == b'B' ||
        c == b'C' ||
        c == b'D' ||
        c == b'E' ||
        c == b'F' ||
        c == b'a' ||
        c == b'b' ||
        c == b'c' ||
        c == b'd' ||
        c == b'e' ||
        c == b'f';
}

pub fn is_oct(c: u8) -> bool {
    return c == b'0' ||
        c == b'1' ||
        c == b'2' ||
        c == b'3' ||
        c == b'4' ||
        c == b'5' ||
        c == b'6' ||
        c == b'7';
}

pub fn is_bin(c: u8) -> bool {
    return c == b'0' ||
        c == b'1';
}

pub fn is_op_start(c: u8) -> bool {
    return c == b'>' ||
        c == b'<' ||
        c == b'-' ||
        c == b'|' ||
        c == b'+' ||
        c == b'&' ||
        c == b'#' ||
        c == b'^' ||
        c == b'=' ||
        c == b'%' ||
        c == b'/' ||
        c == b'!' ||
        c == b'*' ||
        c == b'.' ||
        c == b'~' ||
        c == b'}' ||
        c == b'{' ||
        c == b')' ||
        c == b'(' ||
        c == b']' ||
        c == b'?' ||
        c == b':' ||
        c == b';' ||
        c == b',' ||
        c == b'[';
}

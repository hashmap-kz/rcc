#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

pub struct CBuf {
    buffer: Vec<u8>,
    size: usize,
    offset: usize,
    pub line: i32,
    pub column: i32,
    prev_char: u8,
    eof_cnt: i32,
}

impl CBuf {
    /// We need a mutable reference, because we have to apply some paddings.
    /// These extra bytes are quite useful, because we may check offset like -> offset+2, etc.
    /// And do not care whether we may go out of bounds.
    ///
    pub fn create(string: &String) -> Self {
        let mut res: Vec<u8> = Vec::new();
        for c in string.as_bytes() {
            res.push(*c);
        }
        for _ in 0..8 {
            res.push(b'\0');
        }

        let len = res.len();

        CBuf {
            buffer: res,
            size: len,
            offset: 0,
            line: 1,
            column: 0,
            prev_char: 0,
            eof_cnt: -1,
        }
    }

    pub fn is_eof(&self) -> bool {
        return self.offset >= self.size;
    }

    pub fn next(&mut self) -> u8 {
        while !self.is_eof() {
            if self.eof_cnt > 8 {
                panic!("Infinite loop handling...");
            }

            if self.prev_char == b'\n' {
                self.line += 1;
                self.column = 0;
            }

            if self.buffer[self.offset] == b'\\' {
                if self.buffer[self.offset + 1] == b'\r' {
                    if self.buffer[self.offset + 2] == b'\n' {
                        // DOS: [\][\r][\n]
                        self.offset += 3;
                    } else {
                        // OSX: [\][\r]
                        self.offset += 2;
                    }

                    self.prev_char = b'\n';
                    continue;
                }

                // UNX: [\][\n]
                if self.buffer[self.offset + 1] == b'\n' {
                    self.offset += 2;
                    self.prev_char = b'\n';
                    continue;
                }
            }

            if self.buffer[self.offset] == b'\r' {
                if self.buffer[self.offset + 1] == b'\n' {
                    // DOS: [\r][\n]
                    self.offset += 2;
                } else {
                    // OSX: [\r]
                    self.offset += 1;
                }
                self.prev_char = b'\n';
                return b'\n';
            }

            if self.offset == self.size {
                self.eof_cnt += 1;
                return 0;
            }

            let next = self.buffer[self.offset];
            self.offset += 1;
            self.column += 1;
            self.prev_char = next;

            if next == b'\0' {
                self.eof_cnt += 1;
                return 0;
            }

            return next;
        }

        return 0;
    }

    pub fn peek_1(&mut self) -> u8 {
        let save_offset = self.offset;
        let save_line = self.line;
        let save_column = self.column;
        let save_prev_char = self.prev_char;
        let save_eof_cnt = self.eof_cnt;

        let res = self.next();

        self.offset = save_offset;
        self.line = save_line;
        self.column = save_column;
        self.prev_char = save_prev_char;
        self.eof_cnt = save_eof_cnt;

        return res;
    }

    pub fn peek_3(&mut self) -> [u8; 3] {
        let save_offset = self.offset;
        let save_line = self.line;
        let save_column = self.column;
        let save_prev_char = self.prev_char;
        let save_eof_cnt = self.eof_cnt;

        let res = [self.next(), self.next(), self.next()];

        self.offset = save_offset;
        self.line = save_line;
        self.column = save_column;
        self.prev_char = save_prev_char;
        self.eof_cnt = save_eof_cnt;

        return res;
    }
}

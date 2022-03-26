use std::rc::Rc;

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

impl SourceLoc {
    pub fn new(file: Rc<String>, line: i32, column: i32) -> Self {
        SourceLoc {
            file,
            line,
            column,
        }
    }
}

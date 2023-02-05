use derive_more::Constructor;
use std::fmt::Display;

#[derive(Debug, Constructor, Clone, Copy)]
pub struct Pos {
    pub line: usize,
    pub column: usize,
    pub line_start: usize,
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { line, column, .. } = self;
        write!(f, "{line}:{column}")
    }
}

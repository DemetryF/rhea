use derive_more::Constructor;
use std::fmt::Display;

#[derive(Debug, Constructor, Clone, Copy)]
pub struct Pos {
    pub line: usize,
    pub column: usize,
    pub line_start: usize,
}

impl Default for Pos {
    fn default() -> Self {
        Self {
            line: 1,
            column: 1,
            line_start: 0,
        }
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { line, column, .. } = self;
        write!(f, "{line}:{column}")
    }
}

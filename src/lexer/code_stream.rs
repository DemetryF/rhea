use crate::token::Pos;

#[derive(Debug)]
pub struct CodeStream<'code> {
    pub index: usize,

    pos: Pos,
    code: &'code str,
}

impl<'code> CodeStream<'code> {
    pub fn new(code: &'code str) -> Self {
        Self {
            pos: Pos::default(),
            index: 0,
            code,
        }
    }

    pub fn current(&self) -> char {
        self.code[self.index..].chars().next().unwrap()
    }

    pub fn check(&self, str: &str) -> bool {
        let start = self.index;

        if start + str.len() > self.code.len() {
            return false;
        }

        self.slice(start, str.len()) == str
    }

    pub fn slice(&self, start: usize, len: usize) -> &str {
        &self.code[start..start + len]
    }

    pub fn slice_from_current(&self, len: usize) -> &str {
        &self.code[self.index..self.index + len]
    }

    pub fn skip(&mut self, count: usize) -> &str {
        for _ in 0..count {
            self.accept();
        }

        &self.code[self.index - count..self.index]
    }

    pub fn accept(&mut self) -> char {
        let ch = self.current();

        match ch {
            '\n' => {
                self.pos.line += 1;
                self.pos.column = 1;
                self.pos.line_start = self.index + 1;
            }
            _ => self.pos.column += 1,
        }
        self.index += 1;

        ch
    }

    pub fn is_eof(&self) -> bool {
        self.index >= self.code.len()
    }

    pub fn get_pos(&self) -> Pos {
        self.pos
    }
}

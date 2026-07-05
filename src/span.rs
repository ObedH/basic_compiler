#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub col: usize,
    pub pos: usize,
}

impl Position {
    pub fn new(line: usize, col: usize, pos: usize) -> Self {
        Self { line, col, pos }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: Position,
    pub end: Position,
    pub len: usize,
}
impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end, len: end.pos - start.pos }
    }
}

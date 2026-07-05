use crate::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    If,
    Then,
    Else,
    For,
    While,
    Repeat,
    End,
    Pause,
    Lbl,
    Goto,
    IS,
    DS,
    Menu,
    Prgm,
    Return,
    Stop,
    DelVar,
    GraphStyle,
    Input,
    Prompt,
    Disp,
    DispGraph,
    DispTable,
    Output,
    GetKey,
    ClrHome,
    ClrTable,
    GetCalc,
    Get,
    Send_,

    LParen,
    RParen,
    Comma,
    Dot,
    Minus,
    Plus,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    Xor,
    
    NumberLit(f32),
    StringLit(String),
    Var(char),

    Newline,
    Error(String),
    EOF
}

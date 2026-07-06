use crate::token::TokenKind;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Self { statements: Vec::new() }
    }
    pub fn add_statement(&mut self, statement: Statement) {
        self.statements.push(statement);
    }
    pub fn get_statements(&self) -> Vec<Statement> {
        self.statements.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Display(Option<Expression>),
    Assign {
        target: AssignTarget,
        source: Expression,
    },
    If {
        condition: Expression,
        consequence: Vec<Statement>,
        alternative: Vec<Statement>,
    },
    For {
        variable: RealVar,
        min: Expression,
        max: Expression,
        step: Option<Expression>,
        body: Vec<Statement>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Binary {
        op: BinaryOperator,
        left:  Box<Expression>,
        right: Box<Expression>,
    },
    Unary {
        op: UnaryOperator,
        operand: Box<Expression>,
    },

    NumberLiteral(f32),
    StringLiteral(String),

    RealVariable(RealVar),

    Grouping(Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    And,
    Or,
    Xor,
    None,
}
impl BinaryOperator {
    pub fn from_token(token: TokenKind) -> BinaryOperator {
        match token {
            TokenKind::Minus        => BinaryOperator::Sub,
            TokenKind::Plus         => BinaryOperator::Add,
            TokenKind::Slash        => BinaryOperator::Div,
            TokenKind::Star         => BinaryOperator::Mul,
            TokenKind::Equal        => BinaryOperator::Equal,
            TokenKind::BangEqual    => BinaryOperator::NotEqual,
            TokenKind::Less         => BinaryOperator::Less,
            TokenKind::Greater      => BinaryOperator::Greater,
            TokenKind::LessEqual    => BinaryOperator::LessEqual,
            TokenKind::GreaterEqual => BinaryOperator::GreaterEqual,
            TokenKind::And          => BinaryOperator::And,
            TokenKind::Or           => BinaryOperator::Or,
            TokenKind::Xor          => BinaryOperator::Xor,
            _                       => BinaryOperator::None
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Negative,
    Not,
    None,
}
impl UnaryOperator {
    pub fn from_token(token: TokenKind) -> UnaryOperator {
        match token {
            TokenKind::Minus => UnaryOperator::Negative,
            TokenKind::Bang  => UnaryOperator::Not,
            _                => UnaryOperator::None
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RealVar {
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
}
impl RealVar {
    pub fn from_char(c: char) -> Option<RealVar> {
        match c {
            'A' => Some(RealVar::A),
            'B' => Some(RealVar::B),
            'C' => Some(RealVar::C),
            'D' => Some(RealVar::D),
            'E' => Some(RealVar::E),
            'F' => Some(RealVar::F),
            'G' => Some(RealVar::G),
            'H' => Some(RealVar::H),
            'I' => Some(RealVar::I),
            'J' => Some(RealVar::J),
            'K' => Some(RealVar::K),
            'L' => Some(RealVar::L),
            'M' => Some(RealVar::M),
            'N' => Some(RealVar::N),
            'O' => Some(RealVar::O),
            'P' => Some(RealVar::P),
            'Q' => Some(RealVar::Q),
            'R' => Some(RealVar::R),
            'S' => Some(RealVar::S),
            'T' => Some(RealVar::T),
            'U' => Some(RealVar::U),
            'V' => Some(RealVar::V),
            'W' => Some(RealVar::W),
            'X' => Some(RealVar::X),
            'Y' => Some(RealVar::Y),
            'Z' => Some(RealVar::Z),
            _   => None,
        }
    }
    pub fn to_char(var: RealVar) -> char {
        match var {
            RealVar::A => 'A',
            RealVar::B => 'B',
            RealVar::C => 'C',
            RealVar::D => 'D',
            RealVar::E => 'E',
            RealVar::F => 'F',
            RealVar::G => 'G',
            RealVar::H => 'H',
            RealVar::I => 'I',
            RealVar::J => 'J',
            RealVar::K => 'K',
            RealVar::L => 'L',
            RealVar::M => 'M',
            RealVar::N => 'N',
            RealVar::O => 'O',
            RealVar::P => 'P',
            RealVar::Q => 'Q',
            RealVar::R => 'R',
            RealVar::S => 'S',
            RealVar::T => 'T',
            RealVar::U => 'U',
            RealVar::V => 'V',
            RealVar::W => 'W',
            RealVar::X => 'X',
            RealVar::Y => 'Y',
            RealVar::Z => 'Z',
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssignTarget {
    RealVariable(RealVar),
}

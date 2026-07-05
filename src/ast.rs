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

#[derive(Debug, Clone, PartialEq)]
pub enum AssignTarget {
    RealVariable(RealVar),
}

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
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Display(Expression),
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
    BinaryExpression {
        op: BinaryOperator,
        left:  Box<Expression>,
        right: Box<Expression>,
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

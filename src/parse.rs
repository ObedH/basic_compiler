use crate::token::Token;
use crate::token::TokenKind;
use crate::ast::*;
use std::mem::discriminant;

#[derive(Debug, Clone, PartialEq)]
pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
    pub program_root: Program,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens: tokens, index: 0, program_root: Program::new() }
    }
    pub fn parse(tokens: Vec<Token>) -> Program {
        let mut parser: Parser = Parser::new(tokens);
        parser.parse_program();
        parser.program_root
    }

    fn parse_program(&mut self) {
        while !self.is_at_end() {
            let statement = self.parse_statement();
            match statement {
                Some(stmt) => self.program_root.add_statement(stmt),
                None       => continue,
            };
        }
    }
    fn parse_statement(&mut self) -> Option<Statement> {
        if self.match_token(vec![TokenKind::Disp]) { return self.parse_disp(); }
        if self.match_token(vec![TokenKind::Newline]) { return None; }

        let expr = self.parse_expression();

        if self.match_token(vec![TokenKind::Arrow]) {
            match self.parse_assign_target() {
                Some(v) => return Some(Statement::Assign { target: v, source: match expr {
                        Some(v) => v,
                        None    => return None,
                    }
                }),
                None => return None,
            }
        }

        None
    }
    fn parse_disp(&mut self) -> Option<Statement> {
        let value = self.parse_expression();
        self.consume(TokenKind::Newline, "Expected newline!");

        Some(Statement::Display(value))
    }
    fn parse_expression(&mut self) -> Option<Expression> {
        self.parse_or()
    }
    fn parse_or(&mut self) -> Option<Expression> {
        let mut expr = self.parse_xor();

        while self.match_token(vec![TokenKind::Or]) {
            let operator = self.previous();
            let right = self.parse_xor();
            expr = Some(Expression::Binary { op: BinaryOperator::from_token(operator.kind), left: Box::new(expr.unwrap()), right: Box::new(right.unwrap()) });
        }

        expr
    }
    fn parse_xor(&mut self) -> Option<Expression> {
        let mut expr = self.parse_and();

        while self.match_token(vec![TokenKind::Xor]) {
            let operator = self.previous();
            let right = self.parse_and();
            expr = Some(Expression::Binary { op: BinaryOperator::from_token(operator.kind), left: Box::new(expr.unwrap()), right: Box::new(right.unwrap()) });
        }

        expr
    }
    fn parse_and(&mut self) -> Option<Expression> {
        let mut expr = self.parse_equality();

        while self.match_token(vec![TokenKind::And]) {
            let operator = self.previous();
            let right = self.parse_equality();
            expr = Some(Expression::Binary { op: BinaryOperator::from_token(operator.kind), left: Box::new(expr.unwrap()), right: Box::new(right.unwrap()) });
        }

        expr
    }
    fn parse_equality(&mut self) -> Option<Expression> {
        let mut expr = self.parse_comparison();

        while self.match_token(vec![TokenKind::Equal, TokenKind::BangEqual]) {
            let operator = self.previous();
            let right = self.parse_comparison();
            expr = Some(Expression::Binary { op: BinaryOperator::from_token(operator.kind), left: Box::new(expr.unwrap()), right: Box::new(right.unwrap()) });
        }

        expr
    }
    fn parse_comparison(&mut self) -> Option<Expression> {
        let mut expr = self.parse_term();

        while self.match_token(vec![TokenKind::Greater, TokenKind::GreaterEqual, TokenKind::Less, TokenKind::LessEqual]) {
            let operator = self.previous();
            let right = self.parse_term();
            expr = Some(Expression::Binary { op: BinaryOperator::from_token(operator.kind), left: Box::new(expr.unwrap()), right: Box::new(right.unwrap()) });
        }

        expr
    }
    fn parse_term(&mut self) -> Option<Expression> {
        let mut expr = self.parse_factor();

        while self.match_token(vec![TokenKind::Minus, TokenKind::Plus]) {
            let operator = self.previous();
            let right = self.parse_factor();
            expr = Some(Expression::Binary { op: BinaryOperator::from_token(operator.kind), left: Box::new(expr.unwrap()), right: Box::new(right.unwrap()) });
        }

        expr
    }
    fn parse_factor(&mut self) -> Option<Expression> {
        let mut expr = self.parse_unary();

        while self.match_token(vec![TokenKind::Slash, TokenKind::Star]) {
            let operator = self.previous();
            let right = self.parse_unary();
            expr = Some(Expression::Binary { op: BinaryOperator::from_token(operator.kind), left: Box::new(expr.unwrap()), right: Box::new(right.unwrap()) });
        }

        expr
    }
    fn parse_unary(&mut self) -> Option<Expression> {
        if self.match_token(vec![TokenKind::Minus, TokenKind::Bang]) {
            let operator = self.previous();
            let right = self.parse_unary();
            return Some(Expression::Unary { op: UnaryOperator::from_token(operator.kind), operand: Box::new(right.unwrap()) });
        }
        self.parse_primary()
    }
    fn parse_primary(&mut self) -> Option<Expression> {
        if self.match_token(vec![TokenKind::NumberLit(0.0)]) {
            return match self.previous().kind {
                TokenKind::NumberLit(val) => Some(Expression::NumberLiteral(val)),
                _ => None,
            };
        }
        if self.match_token(vec![TokenKind::StringLit(String::new())]) {
            return match self.previous().kind {
                TokenKind::StringLit(val) => Some(Expression::StringLiteral(val.to_owned())),
                _ => None,
            };
        }
        if self.match_token(vec![TokenKind::Var('A')]) {
            return match self.previous().kind {
                TokenKind::Var(val) => match val {
                    'A'..'Z' => Some(Expression::RealVariable(
                        match RealVar::from_char(val) {
                            Some(var) => var,
                            None      => return None,
                        },
                    )),
                    _ => None,
                },
                _ => None,
            };
        }
        if self.match_token(vec![TokenKind::LParen]) {
            let expr = self.parse_expression();
            self.consume(TokenKind::RParen, "Expected ')' after expression.");
            return Some(Expression::Grouping(Box::new(expr.unwrap())));
        }

        None
    }
    fn parse_assign_target(&mut self) -> Option<AssignTarget> {
        let target = self.advance();
        return match target.kind {
            TokenKind::Var(c) => Some(AssignTarget::RealVariable(match RealVar::from_char(c) {
                    Some(v) => v,
                    None    => return None,
                }
            )),
            _                 => None,
        };
    }

    fn is_at_end(&self) -> bool {
        self.index >= self.tokens.len() || self.peek().kind == TokenKind::EOF
    }
    fn consume(&mut self, kind: TokenKind, message: &str) -> Token {
        if self.check(kind) { return self.advance(); }
        
        panic!("{}", message);
    }
    fn match_token(&mut self, kinds: Vec<TokenKind>) -> bool {
        if self.is_at_end() {
            return false;
        }

        let current_kind = &self.peek().kind;

        for t in kinds {
            if discriminant(current_kind) == discriminant(&t)  {
                self.advance();
                return true;
            }
        }

        false
    }
    fn check(&mut self, token_kind: TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().kind == token_kind
    }
    fn advance(&mut self) -> Token {
        if !self.is_at_end() { self.index += 1; }
        self.previous()
    }
    fn peek(&self) -> Token {
        self.tokens.get(self.index).unwrap().clone()
    }
    fn previous(&self) -> Token {
        self.tokens.get(self.index - 1).unwrap().clone()
    }
}

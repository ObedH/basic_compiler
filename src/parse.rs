use crate::token::Token;
use crate::ast::*;

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
            let statement: Statement = self.parse_statement();
            self.program_root.add_statement(statement);
        }
    }
    fn parse_statement(&mut self) -> Statement {
        
    }

    fn is_at_end(&self) -> bool {
        self.index >= self.tokens.len()
    }
    fn 
}

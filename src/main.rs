use std::fs;
use basic_compiler::token::Token;
use basic_compiler::lexer::Lexer;
use basic_compiler::ast::Program;
use basic_compiler::parse::Parser;

fn main() {
    let file: String = fs::read_to_string("tests/keyword").expect("Should be able to open tests/basic");

    let token_stream: Vec<Token> = Lexer::lex(&file);
    println!("{:?}", 
        token_stream
            .iter()
            .map(|t| t.kind.clone())
            .collect::<Vec<_>>()
    );

    let program: Program = Parser::parse(token_stream);
    println!("{:?}", program);
}

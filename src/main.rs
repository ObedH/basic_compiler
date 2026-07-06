use std::fs;
use basic_compiler::token::Token;
use basic_compiler::lexer::Lexer;
use basic_compiler::ast::Program;
use basic_compiler::parse::Parser;
use basic_compiler::codegen::x86_64::CodeGeneratorX86_64;

fn main() {
    let input_path: &str = "tests/nested_if.b";
    let output_path: &str = "asm/nested_if.s";
    println!("Opening source file: {}", input_path);

    let file: String = fs::read_to_string(input_path).expect(&format!("Should be able to open {}", input_path));

    let token_stream: Vec<Token> = Lexer::lex(&file);
    println!("{:?}", 
        token_stream
            .iter()
            .map(|t| t.kind.clone())
            .collect::<Vec<_>>()
    );
    println!("");

    let program: Program = Parser::parse(token_stream);
    println!("{:?}", program);
    println!("");

    CodeGeneratorX86_64::gen_asm(output_path, program);
    println!("Output:");
    println!("{}", fs::read_to_string(output_path).expect(&format!("Should be able to open {}", output_path)));
}

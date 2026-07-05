use crate::ast::*;
use std::fs::File;
use std::io::{ Write, BufWriter };
use crate::types::Type;

#[derive(Debug)]
pub struct CodeGeneratorX86_64 {
    writer: BufWriter<File>,
    text_label_count: usize,
    data_label_count: usize,
    string_literals: Vec<(String, String)>,
    float_literals: Vec<(String, f32)>,
}

impl CodeGeneratorX86_64 {
    pub fn new(output_path: &str) -> Self {
        let file = File::create(output_path).expect("Failed to create assembly file");
        Self {
            writer: BufWriter::new(file),
            text_label_count: 0,
            data_label_count: 0,
            string_literals: Vec::new(),
            float_literals: Vec::new(),
        }
    }
    pub fn gen_asm(output_path: &str, program: Program) {
        let mut code_gen = CodeGeneratorX86_64::new(output_path);
        code_gen.generate(program);
    }

    fn new_text_label(&mut self) -> String {
        let label = format!(".L{}", self.text_label_count);
        self.text_label_count += 1;
        label
    }
    fn new_str_label(&mut self, val: String) -> String {
        let label = format!(".LC{}", self.data_label_count);
        self.string_literals.push((label.clone(), val));
        self.data_label_count += 1;
        label
    }
    fn new_float_label(&mut self, val: f32) -> String {
        let label = format!(".LC{}", self.data_label_count);
        self.float_literals.push((label.clone(), val));
        self.data_label_count += 1;
        label
    }
    fn generate(&mut self, program: Program) {

        self.string_literals.push((".LC_NUM_FMT".to_string(), "%f\\n".to_string()));
        self.string_literals.push((".LC_STR_FMT".to_string(), "%s\\n".to_string()));

        self.gen_text_section(program);
        self.gen_rodata_section();
    }
    fn gen_text_section(&mut self, program: Program) {
        writeln!(self.writer, ".section .text").unwrap();
        writeln!(self.writer, "\t.global main").unwrap();
        self.gen_program(program);
    }
    fn gen_program(&mut self, program: Program) {
        writeln!(self.writer, "main:").unwrap();
        writeln!(self.writer, "\tpush %rbp").unwrap();
        writeln!(self.writer, "\tmov %rsp, %rbp").unwrap();

        for statement in program.get_statements() {
            self.gen_statement(statement);
        }

        writeln!(self.writer, "\tmov %rbp, %rsp").unwrap();
        writeln!(self.writer, "\tpop %rbp").unwrap();
        writeln!(self.writer, "\tmov $0, %rax").unwrap();
        writeln!(self.writer, "\tret").unwrap();
    }
    fn gen_statement(&mut self, statement: Statement) {
        match statement {
            Statement::Display(val) => self.gen_disp(val),
            _                       => {}
        };
    }
    fn gen_disp(&mut self, val: Option<Expression>) {
        match val {
            Some(_) => {
                let expr_type = self.gen_expr(val);
                match expr_type {
                    Some(t) => match t {
                        Type::Number => self.gen_num_disp(),
                        Type::Str    => self.gen_str_disp(),
                    },
                    None => writeln!(self.writer, "# Error: Expression is none.").unwrap(),
                };
            },
            None => {},
        };
    }
    fn gen_num_disp(&mut self) {
        // Pop value from stack
        writeln!(self.writer, "\tmovsd (%rsp), %xmm0").unwrap();
        writeln!(self.writer, "\taddq $16, %rsp").unwrap();

        writeln!(self.writer, "\tlea .LC_NUM_FMT(%rip), %rdi").unwrap();
        writeln!(self.writer, "\tmov $1, %al").unwrap();
        writeln!(self.writer, "\tcall printf").unwrap();
        writeln!(self.writer, "# Printed a number").unwrap();
    }
    fn gen_str_disp(&mut self) {
        writeln!(self.writer, "\tpop %rsi").unwrap();
        writeln!(self.writer, "\tlea .LC_STR_FMT(%rip), %rdi").unwrap();
        writeln!(self.writer, "\tmov $0, %al").unwrap();
        writeln!(self.writer, "\tcall printf").unwrap();
        writeln!(self.writer, "# Printed a string").unwrap();
    }
    fn gen_expr(&mut self, val: Option<Expression>) -> Option<Type> {
        match val {
            Some(expr) => match expr {
                Expression::Binary { op, left, right }  => self.gen_bin_expr(op, *left, *right),
                Expression::Unary { op, operand }       => self.gen_un_expr(op, *operand),
                Expression::NumberLiteral(x)            => self.gen_num_lit(x),
                Expression::StringLiteral(x)            => self.gen_str_lit(x),
                Expression::RealVariable(var)           => self.gen_real_var(var),
                Expression::Grouping(b)                 => self.gen_expr(Some(*b)),
            },
            None => None,
        }
    }
    fn gen_bin_expr(&mut self, op: BinaryOperator, left: Expression, right: Expression) -> Option<Type> {
        // Evaluate left side
        let left_type = self.gen_expr(Some(left));
        match left_type {
            Some(t) => match t {
                Type::Number => {},
                Type::Str    => return None,
            },
            None => return None,
        };
        // Evaluate right side
        let right_type = self.gen_expr(Some(right));
        match right_type {
            Some(t) => match t {
                Type::Number => {},
                Type::Str    => return None,
            },
            None => return None,
        };
        
        // Pop value from stack into %xmm0
        writeln!(self.writer, "\tmovsd (%rsp), %xmm0").unwrap();
        writeln!(self.writer, "\taddq $16, %rsp").unwrap();

        // Pop value from stack into %xmm1
        writeln!(self.writer, "\tmovsd (%rsp), %xmm1").unwrap();
        writeln!(self.writer, "\taddq $16, %rsp").unwrap();

        // Perform the operation
        match op {
            BinaryOperator::Add         => writeln!(self.writer, "\taddsd %xmm0, %xmm1").unwrap(),
            BinaryOperator::Sub         => writeln!(self.writer, "\tsubsd %xmm0, %xmm1").unwrap(),
            BinaryOperator::Mul         => writeln!(self.writer, "\tmulsd %xmm0, %xmm1").unwrap(),
            BinaryOperator::Div         => writeln!(self.writer, "\tdivsd %xmm0, %xmm1").unwrap(),
            _                           => return None,
        };

        // Push value onto stack
        writeln!(self.writer, "\tsubq $16, %rsp").unwrap();
        writeln!(self.writer, "\tmovsd %xmm1, (%rsp)").unwrap();

        match op {
            BinaryOperator::Add         => writeln!(self.writer, "# Evaluated: Add").unwrap(),
            BinaryOperator::Sub         => writeln!(self.writer, "# Evaluated: Sub").unwrap(),
            BinaryOperator::Mul         => writeln!(self.writer, "# Evaluated: Mul").unwrap(),
            BinaryOperator::Div         => writeln!(self.writer, "# Evaluated: Div").unwrap(),
            _                           => return None,
        };

        Some(Type::Number)
    }
    fn gen_un_expr(&mut self, op: UnaryOperator, operand: Expression) -> Option<Type> {
        None
    }
    fn gen_num_lit(&mut self, val: f32) -> Option<Type> {
        let num_label = self.new_float_label(val);
        // Push value onto stack
        writeln!(self.writer, "\tmovsd {}(%rip), %xmm0", num_label).unwrap();
        writeln!(self.writer, "\tsubq $16, %rsp").unwrap();
        writeln!(self.writer, "\tmovsd %xmm0, (%rsp)").unwrap();
        writeln!(self.writer, "# Evaluated: {}", val).unwrap();
        Some(Type::Number)
    }
    fn gen_str_lit(&mut self, val: String) -> Option<Type> {
        let str_label = self.new_str_label(val.clone());
        writeln!(self.writer, "\tlea {}(%rip), %rax", str_label).unwrap();
        writeln!(self.writer, "\tpush %rax").unwrap();
        writeln!(self.writer, "# Evaluated: \"{}\"", val).unwrap();
        Some(Type::Str)
    }
    fn gen_real_var(&mut self, var: RealVar) -> Option<Type> {
        Some(Type::Number)
    }
    fn gen_rodata_section(&mut self) {
        writeln!(self.writer, "\n.section .rodata").unwrap();
        for (label, value) in &self.string_literals {
            writeln!(self.writer, "{}:", label).unwrap();
            writeln!(self.writer, "\t.string \"{}\"", value).unwrap();
        }
        writeln!(self.writer, ".align 8").unwrap();
        for (label, value) in &self.float_literals {
            writeln!(self.writer, "{}:", label).unwrap();
            writeln!(self.writer, "\t.double {}", value).unwrap();
        }
    }
}

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

        self.string_literals.push((".LC_NUM_PRINT_FMT".to_string(), "%f\\n".to_string()));
        self.string_literals.push((".LC_STR_PRINT_FMT".to_string(), "%s\\n".to_string()));
        self.string_literals.push((".LC_NUM_SCAN_FMT".to_string(), "%lf".to_string()));
        self.new_float_label(1.0);
        
        self.gen_data_section();
        self.gen_bss_section();
        self.gen_text_section(program);
        self.gen_rodata_section();
    }
    fn gen_data_section(&mut self) {
        writeln!(self.writer, ".section .data").unwrap();
        writeln!(self.writer, ".align 8").unwrap();
        for byte in b'A'..b'Z' {
            let letter = byte as char;
            writeln!(self.writer, "VAR_{}:\t.double 0.0", letter).unwrap();
        }
    }
    fn gen_bss_section(&mut self) {
        writeln!(self.writer, "\n.section .bss").unwrap();
        writeln!(self.writer, "\t.comm .LC_NUM_SCAN_BUF, 8, 8").unwrap();
    }
    fn gen_text_section(&mut self, program: Program) {
        writeln!(self.writer, "\n.section .text").unwrap();
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
            Statement::Display(val)                                 => self.gen_disp(val),
            Statement::Prompt(var)                                  => self.gen_prompt(var),
            Statement::Assign { target, source }                    => self.gen_assign(target, source),
            Statement::If { condition, consequence, alternative }   => self.gen_if(condition, consequence, alternative),
            Statement::For { variable, min, max, step, body }            => self.gen_for(variable, min, max, step, body),
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
            BinaryOperator::Equal       => writeln!(self.writer, "\tcmpsd $0, %xmm0, %xmm1\n\tmovsd .LC0(%rip), %xmm2\n\tandpd %xmm2, %xmm1").unwrap(),
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
            BinaryOperator::Equal       => writeln!(self.writer, "# Evaluated: Equal").unwrap(),
            _                           => return None,
        };

        Some(Type::Number)
    }
    fn gen_un_expr(&mut self, op: UnaryOperator, operand: Expression) -> Option<Type> {
        None
    }
    fn gen_num_lit(&mut self, val: f32) -> Option<Type> {
        let num_label = self.new_float_label(val);
        writeln!(self.writer, "\tmovsd {}(%rip), %xmm0", num_label).unwrap();
        // Push value onto stack
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
        // Fetch global variable from .data section
        writeln!(self.writer, "\tmovsd VAR_{}(%rip), %xmm0", RealVar::to_char(var.clone())).unwrap();
        // Now push it onto the stack
        writeln!(self.writer, "\tsubq $16, %rsp").unwrap();
        writeln!(self.writer, "\tmovsd %xmm0, (%rsp)").unwrap();
        writeln!(self.writer, "# Read variable: {}", RealVar::to_char(var.clone())).unwrap();
        Some(Type::Number)
    }
    fn gen_num_disp(&mut self) {
        // Pop value from stack
        writeln!(self.writer, "\tmovsd (%rsp), %xmm0").unwrap();
        writeln!(self.writer, "\taddq $16, %rsp").unwrap();

        writeln!(self.writer, "\tlea .LC_NUM_PRINT_FMT(%rip), %rdi").unwrap();
        writeln!(self.writer, "\tmov $1, %al").unwrap();
        writeln!(self.writer, "\tcall printf").unwrap();
        writeln!(self.writer, "# Printed a number").unwrap();
    }
    fn gen_str_disp(&mut self) {
        writeln!(self.writer, "\tpop %rsi").unwrap();
        writeln!(self.writer, "\tlea .LC_STR_PRINT_FMT(%rip), %rdi").unwrap();
        writeln!(self.writer, "\tmov $0, %al").unwrap();
        writeln!(self.writer, "\tcall printf").unwrap();
        writeln!(self.writer, "# Printed a string").unwrap();
    }
    fn gen_prompt(&mut self, var: AssignTarget) {
        writeln!(self.writer, "\tleaq .LC_NUM_SCAN_FMT(%rip), %rdi").unwrap();
        writeln!(self.writer, "\tleaq .LC_NUM_SCAN_BUF(%rip), %rsi").unwrap();
        writeln!(self.writer, "\tmovb $0, %al").unwrap();
        writeln!(self.writer, "\tcall scanf").unwrap();
        writeln!(self.writer, "\tmovsd .LC_NUM_SCAN_BUF(%rip), %xmm0").unwrap();
        writeln!(self.writer, "\tmovsd %xmm0, VAR_{}(%rip)", RealVar::to_char(match var {
            AssignTarget::RealVariable(v) => v
        })).unwrap();
    }
    fn gen_assign(&mut self, target: AssignTarget, source: Expression) {
        let expr = self.gen_expr(Some(source));
        match expr {
            Some(t) => match t {
                    Type::Number => {},
                    Type::Str    => return,
                },
            None => return,
        };

        // Pop value from stack
        writeln!(self.writer, "\tmovsd (%rsp), %xmm0").unwrap();
        writeln!(self.writer, "\taddq $16, %rsp").unwrap();

        self.gen_assign_target(target);
    }
    fn gen_assign_target(&mut self, target: AssignTarget) {
        match target {
            AssignTarget::RealVariable(real_var) => {
                writeln!(self.writer, "\tmovsd %xmm0, VAR_{}(%rip)", RealVar::to_char(real_var.clone())).unwrap();
                writeln!(self.writer, "# Wrote to variable {}", RealVar::to_char(real_var.clone())).unwrap();
            },
        };
    }
    fn gen_if(&mut self, condition: Expression, consequence: Vec<Statement>, alternative: Vec<Statement>) {
        let expr = self.gen_expr(Some(condition));
        match expr {
            Some(t) => match t {
                Type::Number    => {},
                Type::Str       => return,
            },
            None    => return,
        };

        // Pop value from stack
        writeln!(self.writer, "\tmovsd (%rsp), %xmm0").unwrap();
        writeln!(self.writer, "\taddq $16, %rsp").unwrap();
        
        // Compare xmm0 to 0
        writeln!(self.writer, "# If statement:").unwrap();
        writeln!(self.writer, "\txorpd %xmm1, %xmm1").unwrap();
        writeln!(self.writer, "\tcomisd %xmm0, %xmm1").unwrap();
        let false_label = self.new_text_label();
        let done_label = self.new_text_label();
        writeln!(self.writer, "\tje {}", false_label).unwrap();
        
        // Generate then block
        writeln!(self.writer, "# Then block:").unwrap();
        for statement in consequence {
            self.gen_statement(statement);
        }
        writeln!(self.writer, "\tjmp {}", done_label).unwrap();

        // Generate else block
        writeln!(self.writer, "# Else block:").unwrap();
        writeln!(self.writer, "{}:", false_label).unwrap();
        for statement in alternative {
            self.gen_statement(statement);
        }
        writeln!(self.writer, "# Done:").unwrap();
        writeln!(self.writer, "{}:", done_label).unwrap();
    }
    fn gen_for(&mut self, var: RealVar, min: Expression, max: Expression, step: Option<Expression>, body: Vec<Statement>) {
        let var_char = RealVar::to_char(var.clone());

        // 1. Generate and save MIN value
        if self.gen_expr(Some(min)).is_none() { return; }
        writeln!(self.writer, "\tmovsd (%rsp), %xmm0").unwrap();
        writeln!(self.writer, "\taddq $16, %rsp").unwrap(); // Clear min frame
        writeln!(self.writer, "\tmovsd %xmm0, VAR_{}(%rip)", var_char).unwrap();

        // 2. Generate MAX value (Leaves it on stack at (%rsp))
        if self.gen_expr(Some(max)).is_none() { return; }
        writeln!(self.writer, "\tmovsd (%rsp), %xmm3").unwrap();
        writeln!(self.writer, "\taddq $16, %rsp").unwrap(); // Clear max frame

        // 3. Allocate a uniform 48-byte loop tracking context frame (16-byte aligned)
        writeln!(self.writer, "\tsubq $48, %rsp").unwrap();
        writeln!(self.writer, "\tmovsd %xmm3, 32(%rsp)").unwrap(); // Keep MAX safe at offset 32

        // 4. Generate STEP value and Direction Flag
        match step {
            Some(e) => {
                if self.gen_expr(Some(e)).is_none() { return; }
                writeln!(self.writer, "\tmovsd (%rsp), %xmm4").unwrap();
                writeln!(self.writer, "\taddq $16, %rsp").unwrap(); // Clear step frame
                
                writeln!(self.writer, "\tmovsd %xmm4, 16(%rsp)").unwrap(); // Save STEP at offset 16
                writeln!(self.writer, "\tmovmskpd %xmm4, %eax").unwrap();
                writeln!(self.writer, "\tand $1, %eax").unwrap();
                writeln!(self.writer, "\tmovl %eax, 0(%rsp)").unwrap();   // Save DIRECTION at offset 0
            },
            None => {
                writeln!(self.writer, "\tmovsd .LC0(%rip), %xmm4").unwrap();
                writeln!(self.writer, "\tmovsd %xmm4, 16(%rsp)").unwrap(); // Save Step 1.0 at offset 16
                writeln!(self.writer, "\tmovl $0, 0(%rsp)").unwrap();      // Save Forward (0) at offset 0
            },
        };

        // STACK LAYOUT CONFIGURATION:
        // 0(%rsp)  = Direction integer (4 bytes active, 12 bytes alignment padding)
        // 16(%rsp) = Step float value (8 bytes active, 8 bytes alignment padding)
        // 32(%rsp) = Max float value (8 bytes active, 8 bytes alignment padding)

        let loop_body = self.new_text_label();
        let loop_check = self.new_text_label();
        let neg_label = self.new_text_label();
        let done_label = self.new_text_label();

        writeln!(self.writer, "\tjmp {}", loop_check).unwrap();
        writeln!(self.writer, "# Starting for loop by jumping to check").unwrap();

        // --- LOOP BODY ---
        writeln!(self.writer, "{}:", loop_body).unwrap();
        for statement in body {
            self.gen_statement(statement);
        }

        // --- INCREMENTATION ---
        writeln!(self.writer, "# Incrementation").unwrap();
        writeln!(self.writer, "\tmovsd 16(%rsp), %xmm4").unwrap(); // SAFE RELOAD: step from memory
        writeln!(self.writer, "\tmovsd VAR_{}(%rip), %xmm0", var_char).unwrap();
        writeln!(self.writer, "\taddsd %xmm4, %xmm0").unwrap();
        writeln!(self.writer, "\tmovsd %xmm0, VAR_{}(%rip)", var_char).unwrap();

        // --- CONDITION CHECK ---
        writeln!(self.writer, "# Loop checking").unwrap();
        writeln!(self.writer, "{}:", loop_check).unwrap();

        // Peek direction flag without moving %rsp
        writeln!(self.writer, "\tmovl 0(%rsp), %eax").unwrap(); 
        writeln!(self.writer, "\ttest %eax, %eax").unwrap();
        writeln!(self.writer, "\tjnz {}", neg_label).unwrap();

        // Forward Loop: Continue if Max >= Current Value
        writeln!(self.writer, "\tmovsd 32(%rsp), %xmm3").unwrap();              // SAFE RELOAD: Max
        writeln!(self.writer, "\tmovsd VAR_{}(%rip), %xmm0", var_char).unwrap(); // SAFE RELOAD: Current
        writeln!(self.writer, "\tcomisd %xmm0, %xmm3").unwrap();                 // Compare Max vs Current
        writeln!(self.writer, "\tjae {}", loop_body).unwrap();                   // Jump back if Max >= Current
        writeln!(self.writer, "\tjmp {}", done_label).unwrap();

        // Backward Loop: Continue if Current Value >= Max
        writeln!(self.writer, "{}:", neg_label).unwrap();
        writeln!(self.writer, "\tmovsd 32(%rsp), %xmm3").unwrap();              // SAFE RELOAD: Max
        writeln!(self.writer, "\tmovsd VAR_{}(%rip), %xmm0", var_char).unwrap(); // SAFE RELOAD: Current
        writeln!(self.writer, "\tcomisd %xmm3, %xmm0").unwrap();                 // Compare Current vs Max
        writeln!(self.writer, "\tjae {}", loop_body).unwrap();                   // Jump back if Current >= Max

        // --- LOOP DONE CLEANUP ---
        writeln!(self.writer, "{}:", done_label).unwrap();
        writeln!(self.writer, "\taddq $48, %rsp").unwrap(); // Clear the dedicated loop frame completely
        writeln!(self.writer, "# Loop done").unwrap();
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

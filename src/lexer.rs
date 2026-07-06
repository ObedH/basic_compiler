use crate::token::Token;
use crate::token::TokenKind;
use crate::span::Position;
use crate::span::Span;


pub struct Lexer<'a> {
    input: &'a str,
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    pos: Position,
    start: Position,
    tokens: Vec<Token>,
}
impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, chars: input.chars().peekable(), pos: Position::new(1, 1, 0), start: Position::new(1, 1, 0), tokens: Vec::new() }
    }
    pub fn is_at_end(&self) -> bool {
        self.pos.pos >= self.input.len()
    }
    pub fn lex(input: &'a str) -> Vec<Token> {
        let mut lexer: Lexer = Lexer::new(input);

        while !lexer.is_at_end() {
            lexer.start = lexer.pos;
            lexer.scan_token();
        }

        lexer.tokens.push(Token::new(TokenKind::EOF, Span::new(lexer.pos, lexer.pos)));

        lexer.tokens
    }
    pub fn scan_token(&mut self) {
        let Some(c) = self.advance() else { return; };
        
        let kind = match c {
            '('      => TokenKind::LParen,
            ')'      => TokenKind::RParen,
            ','      => TokenKind::Comma,
            '.'      => TokenKind::Dot,
            '+'      => TokenKind::Plus,
            '*'      => TokenKind::Star,
            '/'      => TokenKind::Slash,
            '='      => TokenKind::Equal,
            '\n'     => TokenKind::Newline,
            '-'      => if self.match_char('>') { TokenKind::Arrow }   else { TokenKind::Minus },
            '!'      => if self.match_char('=') { TokenKind::BangEqual }    else { TokenKind::Bang },
            '>'      => if self.match_char('=') { TokenKind::GreaterEqual } else { TokenKind::Greater },
            '<'      => if self.match_char('=') { TokenKind::LessEqual }    else { TokenKind::Less },
            '"'      => self.scan_string(),
            ' '      => { self.skip_whitespace(); return; },
            '0'..'9' => { self.scan_number() },
            _ if c.is_alphabetic() => self.scan_keyword(),
            _        => TokenKind::Error(self.current_lexeme().to_owned()),
        };
    
        self.add_token(kind);
    }
    
    fn advance(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        self.pos.pos += c.len_utf8();
        if c == '\n' {
            self.pos.line += 1;
            self.pos.col = 1;
        }
        else {
            self.pos.col += 1;
        }

        Some(c)
    }
    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }
    fn peek_next(&mut self) -> Option<char> {
        let remaining = &self.input[self.pos.pos..];
        remaining.chars().nth(1)
    }
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {return false;}
        if *self.peek().expect("Should be a char!") != expected {return false;}
        self.advance();
        return true;
    }
    fn current_lexeme(&self) -> &'a str {
        &self.input[self.start.pos..self.pos.pos]
    }
    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            }
            else {
                break;
            }
        }
    }
    fn scan_string(&mut self) -> TokenKind {
        while *self.peek().unwrap() != '"' && !self.is_at_end() {
            self.advance();
        }
        if self.is_at_end()  {
            return TokenKind::Error(self.current_lexeme().to_owned());
        }
        self.advance(); // The final "
        
        let value: &'a str = self.current_lexeme();
        let mut chars = value.chars();
        chars.next();
        chars.next_back();
        TokenKind::StringLit(chars.as_str().to_owned())
    }
    fn scan_number(&mut self) -> TokenKind {
        while (*self.peek().expect("Should be a char!")).is_ascii_digit() { self.advance(); }
        if *self.peek().expect("Should be a char!") == '.' && (self.peek_next().expect("Should be a char!")).is_ascii_digit() {
            self.advance();
    
            while (*self.peek().expect("Should be a char!")).is_ascii_digit() { self.advance(); }
        }
        TokenKind::NumberLit(self.current_lexeme().parse::<f32>().unwrap())
    }
    fn scan_keyword(&mut self) -> TokenKind {
        while (*self.peek().expect("Should be a char!")).is_alphabetic() { self.advance(); }
        
        let keyword: &str = self.current_lexeme();
        
        match keyword {
            "If"    => TokenKind::If,
            "Then"  => TokenKind::Then,
            "Else"  => TokenKind::Else,
            "For"   => TokenKind::For,
            "While" => TokenKind::While,
            "Repeat"=> TokenKind::Repeat,
            "End"   => TokenKind::End,
            "Pause" => TokenKind::Pause,
            "Lbl"   => TokenKind::Lbl,
            "Goto"  => TokenKind::Goto,
            "Disp"  => TokenKind::Disp,
            "And"   => TokenKind::And,
            "Or"    => TokenKind::Or,
            "Xor"   => TokenKind::Xor,
            _ if keyword.len() == 1 && keyword.chars().next().map_or(false, |c| c.is_uppercase()) => TokenKind::Var(keyword.chars().next().unwrap()),
            _       => TokenKind::Error(keyword.to_owned()),
        }
    }
    fn add_token(&mut self, token_type: TokenKind) {
        self.tokens.push(Token::new(token_type, Span::new(self.start, self.pos)));
    }
}

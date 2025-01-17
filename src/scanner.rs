use std::collections::HashMap;

#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

#[derive(Debug, Clone)]
pub struct Token {
    tokentype: TokenType,
    lexeme: String,
    literal: Literals,
    line: usize, 
}

#[derive(Debug, Clone)]
enum Literals {
    Identifier(String),
    String(String),
    Number(f64),
    Null
}

#[derive(Debug, PartialEq, Clone)]
enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

impl Token {
    pub fn new(tokentype: TokenType, lexeme: String, literal: Literals, line: usize) -> Self {
        Self { tokentype, lexeme, literal, line }
    }

    pub fn toString(&self) -> String {
        format!("{:?} {:?} {:?}", self.tokentype, self.lexeme, self.literal)
    }
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self 
        {   source: source, 
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: Scanner::initialize_keywords()
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }

        let token = Token::new(
            TokenType::Eof,
            "".to_string(),
            Literals::Null,
            self.line
        );

        self.tokens.push(token);
        return self.tokens.clone();
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.get((self.current)..self.current+1).unwrap() != &expected.to_string() {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
            let token_type = if self.match_next('=') {
                TokenType::BangEqual
            } else {
                TokenType::Bang
            };
            self.add_token(token_type);
            }
            '=' => {
                let token_type = if self.match_next('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.match_next('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.match_next('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type);
            }   
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && self.is_at_end() {
                        self.advance();
                    }
                }  else {
                    self.add_token(TokenType::Slash)
                }
            } 
            ' ' | '\r' | '\t' => {
            }
            '\n' => {
                self.line += 1;
            }
            '"' => {
                self.string()
            }
            _ => {
                if self.is_digit(c){
                    self.number()
                }
                else if self.is_alpha(c) {
                    self.identifier()
                } else  {
                    eprintln!("Unexpected character.")

                }
            }
        }
    }

    // fn advance(&mut self) -> &str {
    //     let value = self.source.get((self.current)..self.current+1).unwrap();
    //     self.current += 1;
    //     return value;
    // }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap_or('\0')
    }

    fn add_token(&mut self, tokentype: TokenType) {
        self.add_token2(tokentype, Literals::Null);
    }

    fn is_digit(&self, c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text = self.source.get(self.start..self.current).unwrap();
        let mut token_type = self.keywords.get(&text.to_string());
        if token_type ==  None {
            token_type = Some(&TokenType::Identifier);
        }

        let token_type = token_type.unwrap();
        self.add_token(token_type.clone());
    }

    fn is_alpha(&self, c: char) -> bool{
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        return self.is_alpha(c) || self.is_digit(c);
    }


    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            eprintln!("Unterminated string.");
        }
        self.advance();
        let value: String = self.source.get(self.start+1..self.current-1).unwrap().to_string();
        self.add_token2(TokenType::String, Literals::String(value));
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            } 
        }

        let value = match self.source.get(self.start..self.current) {
            Some(value) => {
                value.parse::<f64>().unwrap()
            },
            None => {
                eprintln!("Error Number");
                0.0
            }
        };
        self.add_token2(TokenType::Number, Literals::Number(value))
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        } 
        return self.source.chars().nth(self.current).unwrap();
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        } else {
            return self.source.chars().nth(self.current + 1).unwrap();
        }
    }


    fn add_token2(&mut self, tokentype: TokenType, literal: Literals) {
        let text = self.source.get(self.start..self.current).unwrap();
        self.tokens.push(
            Token{
                tokentype: tokentype,
                lexeme: text.to_string(),
                literal: literal,
                line: self.line, 
            }
        );
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn initialize_keywords() -> HashMap<String, TokenType> {
        let mut keywords = HashMap::new();
        keywords.insert("and".to_string(),    TokenType::And);
        keywords.insert("class".to_string(),  TokenType::Class);
        keywords.insert("else".to_string(),   TokenType::Else);
        keywords.insert("false".to_string(),  TokenType::False);
        keywords.insert("for".to_string(),    TokenType::For);
        keywords.insert("fun".to_string(),    TokenType::Fun);
        keywords.insert("if".to_string(),     TokenType::If);
        keywords.insert("nil".to_string(),    TokenType::Nil);
        keywords.insert("or".to_string(),     TokenType::Or);
        keywords.insert("print".to_string(),  TokenType::Print);
        keywords.insert("return".to_string(), TokenType::Return);
        keywords.insert("super".to_string(),  TokenType::Super);
        keywords.insert("this".to_string(),   TokenType::This);
        keywords.insert("true".to_string(),   TokenType::True);
        keywords.insert("var".to_string(),    TokenType::Var);
        keywords.insert("while".to_string(),  TokenType::While);

        keywords
    }
}

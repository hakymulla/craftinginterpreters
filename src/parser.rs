use std::iter::MapWhile;

use crate::scanner::{TokenType};
use crate::token::Token;
use crate::generate_ast::{Expr, LiteralsAst};

#[derive(Debug)]
struct ParseError;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

#[derive(Debug)]
pub enum Stmt {
    Expression {expression: Expr},
    Print {expression: Expr},
    Var {name: Token, initializer: Expr}
}

impl Parser { 
    pub fn new(tokens: Vec<Token>) -> Self {
       Self {
            tokens: tokens,
            current: 0
       }
    }

    pub fn expression(&mut self) -> Expr {
        println!("expression");
        return self.equality();
    }

    fn equality(&mut self) -> Expr {
        // println!("equality");

        let mut expression: Expr = self.comparison();
        while self.match_token_type(vec![TokenType::BangEqual, TokenType::EqualEqual]).unwrap() {
            let operator = self.previous();
            let right: Expr = self.comparison();

            expression = *Box::new(Expr::Binary 
                {   left: Box::new(expression), 
                    operator: operator, 
                    right:Box::new(right) 
                } 
            );

        };
        // println!("expression: after {:?}", expression);
        return expression;
    }

    fn comparison(&mut self) -> Expr {
        // println!("comparison");

        let mut expression: Expr = self.term();
        while self.match_token_type(vec![TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]).unwrap() {
            let operator = self.previous();
            let right: Expr = self.term();

            expression = *Box::new(Expr::Binary 
                {   left: Box::new(expression), 
                    operator: operator, 
                    right:Box::new(right) 
                } 
            );

        };
        // println!("expression: comparison after {:?}", expression);

        return expression;
    }

    fn match_token_type(&mut self, tokens: Vec<TokenType>) -> Result<bool, ParseError> {
        for token in tokens {
            if self.check(token) {
                self.advance();
                return Ok(true);
            }
        }
        return Ok(false);
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().tokentype == token_type;
    }

    fn previous(&self) -> Token {
        return self.tokens[self.current - 1].clone();
    }

    fn advance(&mut self) -> Result<Token, ParseError> {
        if !self.is_at_end() {
            self.current += 1;
        }
        return Ok(self.previous());
    }

    fn is_at_end(&self) -> bool {
        return self.peek().tokentype == TokenType::Eof;
    }

    fn peek(&self) -> Token {
        return self.tokens[self.current].clone();
    }

    fn term(&mut self) -> Expr {
        // println!("term");
        let mut expression: Expr = self.factor();
        while self.match_token_type(vec![TokenType::Minus, TokenType::Plus]).unwrap() {
            let operator = self.previous();
            let right: Expr = self.factor();

            expression = *Box::new(Expr::Binary 
                {   left: Box::new(expression), 
                    operator: operator, 
                    right:Box::new(right) 
                } 
            );

        };
        // println!("expression: term after {:?}", expression);

        return expression;

    }

    fn factor(&mut self) -> Expr  {
        // println!("factor");

        let mut expression = self.unary().unwrap();
        while self.match_token_type(vec![TokenType::Slash, TokenType::Star]).unwrap() {
            let operator = self.previous();
            let right: Expr = self.unary().unwrap();

            expression = *Box::new(Expr::Binary 
                {   left: Box::new(expression), 
                    operator: operator, 
                    right:Box::new(right) 
                } 
            );

        };
        // println!("expression: factor after {:?}", expression);

        return expression;
    }
    
    fn unary(&mut self) -> Result<Expr, String>  {
        // println!("unary");

        if self.match_token_type(vec![TokenType::Bang, TokenType::Minus]).unwrap() {
            let operator = self.previous();
            let right: Expr = self.unary()?;

            let expression = *Box::new(Expr::Unary
                {   operator: operator, 
                    right:Box::new(right) 
                } 
            );
            // println!("expression: unary {:?}", expression);

            return Ok(expression);
        } else {
            return self.primary();
        }
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.match_token_type(vec![TokenType::False]).unwrap() {
            return Ok(Expr::Literal { value:  LiteralsAst::False});
        }

        if self.match_token_type(vec![TokenType::True]).unwrap() {
            return Ok(Expr::Literal { value: LiteralsAst::True });
        }

        if self.match_token_type(vec![TokenType::Nil]).unwrap() {
            return Ok(Expr::Literal { value: LiteralsAst::Null });
        }

        if self.match_token_type(vec![TokenType::String]).unwrap() {
            return Ok(Expr::Literal { value: LiteralsAst::Strings(self.previous().literal.to_string()) });
        }

        if self.match_token_type(vec![TokenType::Number]).unwrap() {
            let output = self.previous().literal.to_string().parse().unwrap();
            return Ok(Expr::Literal { value: LiteralsAst::Number(output) });
        }

        if self.match_token_type(vec![TokenType::Identifier]).unwrap() {
            return Ok(Expr::Variable{ name: self.previous()});
        }

        if self.match_token_type(vec![TokenType::LeftParen]).unwrap() {
            let expression = self.expression();
            let _ = self.consume(TokenType::RightParen, "Expect ')' after expression.".to_string());
            return Ok(Expr::Grouping { expression: Box::new(expression) });
        }

        else {
            return Err(self.report_error(&self.peek(), &"Expect expression.".to_string()));
        }
    }

    fn consume(&mut self, token_type: TokenType, message: String) -> Result<Token, String> {
        if self.check(token_type) {
            return Ok(self.advance().unwrap());
        }
        return Err(self.report_error(&self.peek(), &message));
        // Err(ParseError)
    }

    fn report_error(&self, token: &Token, message: &str) -> String {
        if token.tokentype == TokenType::Eof {
            return format!("[line {}] Error at end: {}", token.line, message);
        } else {
            return format!("[line {}] Error at '{}': {}", token.line, token.lexeme, message);
        }
    }

    fn synchronize(&mut self) {
        let _ = self.advance();

        while self.is_at_end() {
            if self.previous().tokentype == TokenType::Semicolon {
                return;
            }
            match (self.peek().tokentype) {
                TokenType::Fun | TokenType::Class | TokenType::Var | 
                TokenType::For | TokenType::If | TokenType::While | 
                TokenType::Print | TokenType::Return => return,
                _ => todo!()
            }
        }
        let _ = self.advance();
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration().unwrap());
        }
        return statements;
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token_type(vec![TokenType::Print]).unwrap() {
            return Ok(self.print_statement());
        }
        return Ok(self.expression_statement());
    }

    fn print_statement(&mut self) -> Stmt {
        let value: Expr = self.expression();
        let _ = self.consume(TokenType::Semicolon, "Expect ';' after value.".to_string());
        return Stmt::Print { expression: value };
    }


    fn expression_statement(&mut self) -> Stmt {
        let expr: Expr = self.expression();
        let _ = self.consume(TokenType::Semicolon, "Expect ';' after expression.".to_string());
        return Stmt::Expression { expression: expr };
    }

    fn declaration(&mut self) -> Option<Stmt> {
        let result = (|| {
            if self.match_token_type(vec![TokenType::Var]).unwrap() {
                return self.var_declaration();
            }
            self.statement()
        })();
    
        match result {
            Ok(stmt) => Some(stmt),
            Err(_) => {
                self.synchronize();
                None
            }
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.".to_string()).unwrap();
        let mut initializer= Expr::Null ;
        if self.match_token_type(vec![TokenType::Equal]).unwrap() {
            initializer = self.expression();
        }

        let _ = self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.".to_string());   
        return  Ok(Stmt::Var { name, initializer});
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::scanner::Scanner;
    #[test]
    fn test_comp() {
        let source = "1 + 5 == 2 + 2;".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let parse= parser.parse();

        // assert_eq!(parse.to_string(), "(== (+ 1 5) (+ 2 2))");
    }

    #[test]
    fn test_comp_paren() {
        let source = "(2 + 4 +6) != (5 +7 * 2);".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let parse= parser.parse();

        // assert_eq!(parse.to_string(), "(!= (group (+ (+ 2 4) 6)) (group (+ 5 (* 7 2))))");
    }

    #[test]
    fn test_comp_paren_2() {
        let source = "1 == (2);".to_string();
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let parse = parser.parse();

        // assert_eq!(parse.to_string(), "(== 1 (group 2))");
    }
}


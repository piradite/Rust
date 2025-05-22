use std::io::{self, Write};

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    OpenParent,
    CloseParent,
    Eof
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        if let Some(last) = tokens.last() {
            let prev_is_operand = matches!(last, Token::Number(_) | Token::CloseParent);
            let curr_is_starting_operand = c == '(' || c.is_ascii_digit() || c == '.';
            if prev_is_operand && curr_is_starting_operand {
                tokens.push(Token::Star);
            }
        }

        match c {
            '0'..='9' | '.' => {
                let mut s = String::new();
                let mut has_dot = false;
                while let Some(&ch) = chars.peek() {
                    if ch.is_ascii_digit() {
                        s.push(ch);
                        chars.next();
                    } else if ch == '.' && !has_dot {
                        has_dot = true;
                        s.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(s.parse().unwrap()));
            }
            '+' => { tokens.push(Token::Plus); chars.next(); }
            '-' => { tokens.push(Token::Minus); chars.next(); }
            '*' => { tokens.push(Token::Star); chars.next(); }
            '/' => { tokens.push(Token::Slash); chars.next(); }
            '^' => { tokens.push(Token::Caret); chars.next(); }
            '(' => { tokens.push(Token::OpenParent); chars.next(); }
            ')' => { tokens.push(Token::CloseParent); chars.next(); }
            ' ' | '\t' | '\n' | '\r' => { chars.next(); }
            _ => { chars.next(); }
        }
    }
    tokens.push(Token::Eof);
    tokens
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn curr(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn consume(&mut self) {
        self.pos += 1;
    }

    fn parse(&mut self) -> f64 {
        self.expr()
    }

    fn expr(&mut self) -> f64 {
        let mut left = self.term();
        loop {
            let current_token = self.curr();
            if matches!(current_token, Token::Plus | Token::Minus) {
                let op = current_token.clone();
                self.consume();
                let right = self.term();

                left = match op {
                    Token::Plus => left + right,
                    Token::Minus => left - right,
                    _ => unreachable!(),
                };
            } else {
                break;
            }
        }
        left
    }
    
    fn term(&mut self) -> f64 {
        let mut left = self.factor();
        loop {
            let current_token = self.curr();
            if matches!(current_token, Token::Star | Token::Slash) {
                let op = current_token.clone();
                self.consume();
                let right = self.factor();

                left = match op {
                    Token::Star => left * right,
                    Token::Slash => left / right,
                    _ => unreachable!(),
                };
            } else {
                break;
            }
        }
        left
    }
    
    fn factor(&mut self) -> f64 {
        let left = self.unary();
        if let Token::Caret = self.curr() {
            self.consume();
            let right = self.factor(); 
            left.powf(right)
        } else {
            left
        }
    }

    fn unary(&mut self) -> f64 {
        let mut sign = 1.0;
        loop {
            let current_token = self.curr();
            if matches!(current_token, Token::Plus | Token::Minus) {
                if let Token::Minus = current_token {
                    sign = -sign;
                }
                self.consume(); 
            } else {
                break;
            }
        }
        sign * self.primary()
    }

    fn primary(&mut self) -> f64 {
        match self.curr() {
            Token::Number(n) => {
                let val = *n; 
                self.consume(); 
                val
            }
            Token::OpenParent => {
                self.consume(); 
                let val = self.expr(); 
                self.consume(); 
                val
            }
            _ => {
                panic!("{:?}", self.curr());
            }
        }
    }
}

fn main() {

    loop {
        io::stdout().flush().unwrap();

        let mut expr = String::new();
        io::stdin().read_line(&mut expr).unwrap();
        let expr = expr.trim();

        let tokens = tokenize(expr);
        if tokens.len() == 1 && matches!(tokens[0], Token::Eof) {
            continue;
        }

        let mut parser = Parser::new(tokens);
        let result = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| parser.parse())) {
            Ok(val) => val,
            Err(_) => {
                continue;
            }
        };

        println!("Result: {}", result);

        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        if choice.trim().to_lowercase() == "n" {
            break;
        }
    }
}
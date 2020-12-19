#[derive(Debug, Clone, Eq, PartialEq)]
enum TokenType {
    LeftParen,
    RightParen,
    Minus,
    Plus,
    Slash,
    Star,

    Number,
    Eof,
    Nil,
}

#[derive(Debug, Clone)]
enum Literal {
    Numeric(i64),
    Nil,
}


#[derive(Debug, Clone)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
}


struct Scanner {
    tokens: Vec<Token>,
    start: usize,
    current: usize,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner {
            tokens: Vec::new(),
            start: 0,
            current: 0,
        }
    }

    fn scan_tokens(&mut self, source: &str) -> Vec<Token> {
        while !self.is_at_end(source) {
            self.start = self.current;
            self.scan_token(source);
        }
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            literal: Literal::Nil,
        });
        self.tokens.clone()
    }

    fn is_at_end(&self, source: &str) -> bool {
        return self.current >= source.len();
    }


    fn scan_token(&mut self, source: &str) {
        let c = self.advance(source);

        match c {
            '(' => self.add_token(TokenType::LeftParen, Literal::Nil, source),
            ')' => self.add_token(TokenType::RightParen, Literal::Nil, source),

            '-' => self.add_token(TokenType::Minus, Literal::Nil, source),
            '+' => self.add_token(TokenType::Plus, Literal::Nil, source),
            '*' => self.add_token(TokenType::Star, Literal::Nil, source),
            '/' => self.add_token(TokenType::Slash, Literal::Nil, source),
            '\n' => {}
            _ => {
                if c.is_digit(10) {
                    self.number(source);
                }
                // Throw error
            }
        }
    }

    fn number(&mut self, source: &str) {
        while self.peek(source).is_digit(10) {
            self.advance(source);
        }
        if self.peek(source) == '.' && self.peek_next(source).is_digit(10) {
            self.advance(source);
            while self.peek(source).is_digit(10) {
                self.advance(source);
            }
        }
        let value = &source[self.start..self.current];
        let num = num_from_string(value);
        self.add_token(TokenType::Number, Literal::Numeric(num), source);
    }

    fn peek(&self, source: &str) -> char {
        if self.is_at_end(source) {
            return '\0';
        } else {
            //TODO: Is this really the best way to index this if I KNOW its ascii?
            return source.as_bytes()[self.current] as char;
        }
    }

    fn peek_next(&self, source: &str) -> char {
        if self.current + 1 >= source.len() {
            return '\0';
        } else {
            //TODO: Is this really the best way to index this if I KNOW its ascii?
            return source.as_bytes()[self.current + 1] as char;
        }
    }
    fn match_next(&mut self, expected: char, source: &str) -> bool {
        if self.is_at_end(source) {
            return false;
        }
        if source.as_bytes()[self.current] as char != expected {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn advance(&mut self, source: &str) -> char {
        self.current += 1;
        return source.as_bytes()[self.current - 1] as char;
    }

    fn add_token(&mut self, ttype: TokenType, literal: Literal, source: &str) {
        let text = &source[self.start..self.current];
        self.tokens.push(Token {
            token_type: ttype,
            lexeme: String::from(text),
            literal: literal,
        })
    }
}


fn num_from_string(input_str: &str) -> i64 {
    //println!("{}", input_str);
    let val: i64 = input_str.parse().unwrap();
    return val;
}

#[derive(Debug)]
enum Expression {
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },

    Literal {
        value: Literal,
    },
    Grouping {
        expression: Box<Expression>,
    },
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new() -> Parser {
        Parser {
            tokens: Vec::new(),
            current: 0,
        }
    }
    fn expression(&mut self) -> Expression {
        return self.addition();
    }

    fn addition(&mut self) -> Expression {
        let mut expr = self.unary();

        while self.matcher(&vec![TokenType::Minus, TokenType::Plus, TokenType::Slash, TokenType::Star]) {
            expr = Expression::Binary {
                left: Box::new(expr),
                operator: self.previous().clone(),
                right: Box::new(self.unary()),
            }
        }
        return expr;
    }

    fn unary(&mut self) -> Expression {
        if self.matcher(&vec![TokenType::Minus]) {
            let expr = Expression::Unary {
                operator: self.previous().clone(),
                right: Box::new(self.unary()),
            };

            return expr;
        }
        return self.primary();
    }

    fn primary(&mut self) -> Expression {
        if self.matcher(&vec![TokenType::Nil]) {
            return Expression::Literal {
                value: Literal::Nil,
            };
        } else if self.matcher(&vec![TokenType::Number]) {
            return Expression::Literal {
                value: self.previous().literal.clone(),
            };
        } else if self.matcher(&vec![TokenType::LeftParen]) {
            let expr = self.expression();
            self.consume(
                &TokenType::RightParen,
                "Expect ')' after expression.",
            );
            return Expression::Grouping {
                expression: Box::new(expr),
            };
        } else {
            panic!();
        }
    }

    // helper
    fn matcher(&mut self, ttypes: &Vec<TokenType>) -> bool {
        for ttype in ttypes.iter() {
            if self.check(ttype) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn check(&self, ttype: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().token_type == *ttype;
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }
    fn is_at_end(&self) -> bool {
        return self.peek().token_type == TokenType::Eof;
    }
    fn peek(&self) -> &Token {
        return &self.tokens[self.current];
    }
    fn previous(&self) -> &Token {
        return &self.tokens[self.current - 1];
    }
    fn consume(&mut self, ttype: &TokenType, error_msg: &str) -> &Token {
        if self.check(ttype) {
            return self.advance();
        } else {
            panic!();
        }
    }
}

fn evaluate(expr: &Expression) -> i64 {
    match expr {
        Expression::Literal { value } => {
            match value {
                Literal::Nil => { panic!("found nil") }
                Literal::Numeric(num) => { *num }
            }
        }
        Expression::Grouping { expression } => { evaluate(expression) }

        Expression::Binary { left, operator, right } => {
            let l = evaluate(left);
            let r = evaluate(right);
            match operator.token_type {
                TokenType::Plus => { l + r }
                TokenType::Star => { l * r }
                _ => { panic!("operator not implemented"); }
            }
        }

        Expression::Unary { operator: _operator, right: _right } => { panic!("No Unary"); }
    }
}

fn main() {
    let input_str = include_str!("../input/input_1.txt");

    let mut sum =0;
    for line in input_str.lines() {
        let mut scanner = Scanner::new();
        let mut tokens = scanner.scan_tokens(line);

        // println!("{:?}", tokens);

        let mut parser = Parser::new();
        parser.tokens = tokens.clone();

        let ast = parser.expression();
        // println!("{:?}", ast);

        let final_val = evaluate(&ast);
        sum +=final_val;
        println!("final val {}", final_val);
    }

    println!("sum {}", sum);
}

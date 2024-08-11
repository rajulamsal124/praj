use crate::lexer::token::Token;

pub struct Lexer {
    source: String,
    start: usize,
    current: usize,
    errors: Vec<String>,  // Collect errors instead of panicking
}

impl Lexer {
    // Create a new lexer with the given source code
    pub fn new(source: String) -> Self {
        Lexer {
            source,
            start: 0,
            current: 0,
            errors: Vec::new(),
        }
    }

    // This will return a list of tokens by scanning through the source code
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Some(token) => tokens.push(token),
                None => {} // Skip over None, which represents whitespace, comments, etc.
            }
        }

        tokens.push(Token::EOF);

        // Print any errors collected during lexing
        if self.errors.is_empty() {
            println!("Build success");
        } else {
            for error in &self.errors {
                println!("Error: {}", error);
            }
        }

        tokens
    }

    // This function will scan a single token
    fn scan_token(&mut self) -> Option<Token> {
        let c = self.advance();

        match c {
            '(' => Some(Token::LeftParen),
            ')' => Some(Token::RightParen),
            '{' => Some(Token::LeftBrace),
            '}' => Some(Token::RightBrace),
            ',' => Some(Token::Comma),
            '.' => Some(Token::Dot),
            '-' => Some(Token::Minus),
            '+' => Some(Token::Plus),
            ';' => Some(Token::Semicolon),
            '*' => Some(Token::Star),
            '/' => {
                if self.peek() == '/' {
                    // Skip comments
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    None // Ignore comments
                } else {
                    Some(Token::Slash)
                }
            },
            ' ' | '\r' | '\t' | '\n' => None, // Skip whitespace
            '"' => Some(self.string()),       // Handle string literals
            '0'..='9' => Some(self.number()), // Handle numbers
            'a'..='z' | 'A'..='Z' | '_' => Some(self.identifier()), // Handle identifiers and keywords
            _ => {
                self.errors.push(format!("Unexpected character: '{}'", c));
                None
            }
        }
    }

    // Advance the current position in the source code and return the character
    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap_or('\0');
        self.current += 1;
        c
    }

    // Check if we've reached the end of the source code
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // Handle string literals
    fn string(&mut self) -> Token {
        let mut value = String::new();
        while !self.is_at_end() && self.peek() != '"' {
            value.push(self.advance());
        }

        if self.is_at_end() {
            self.errors.push("Unterminated string literal".to_string());
            return Token::StringLiteral(value); // Return the partially parsed string
        }

        self.advance(); // Consume the closing "
        Token::StringLiteral(value)
    }

    // Handle numeric literals
    fn number(&mut self) -> Token {
        let mut value = String::new();
        
        // Collect digits before the decimal point
        while !self.is_at_end() && self.peek().is_digit(10) {
            value.push(self.advance());
        }

        // Look for a fractional part
        if self.peek() == '.' {
            // Check if it's followed by another digit to form a valid float
            if self.peek_next().is_digit(10) {
                value.push(self.advance()); // Consume the '.'

                // Collect digits after the decimal point
                while !self.is_at_end() && self.peek().is_digit(10) {
                    value.push(self.advance());
                }
            } else {
                self.errors.push("Invalid number literal: lone decimal point".to_string());
                return Token::Number(0.0); // Return a default value
            }
        }

        // Check if the value is non-empty before parsing
        if value.is_empty() || value == "." {
            self.errors.push(format!("Invalid number literal: '{}'", value));
            Token::Number(0.0) // Return a default value
        } else {
            match value.parse() {
                Ok(num) => Token::Number(num),
                Err(_) => {
                    self.errors.push(format!("Failed to parse number: '{}'", value));
                    Token::Number(0.0) // Return a default value
                }
            }
        }
    }

    // Handle identifiers and keywords
    fn identifier(&mut self) -> Token {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        match text {
            "and" => Token::And,
            "class" => Token::Class,
            "else" => Token::Else,
            "false" => Token::False,
            "for" => Token::For,
            "fun" => Token::Fun,
            "if" => Token::If,
            "nil" => Token::Nil,
            "or" => Token::Or,
            "print" => Token::Print,
            "return" => Token::Return,
            "super" => Token::Super,
            "this" => Token::This,
            "true" => Token::True,
            "var" => Token::Var,
            "while" => Token::While,
            _ => Token::Identifier(text.to_string()),
        }
    }

    // Peek at the next character without advancing the current position
    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or('\0')
    }

    // Peek at the character after the next one without advancing
    fn peek_next(&self) -> char {
        self.source.chars().nth(self.current + 1).unwrap_or('\0')
    }
}



    
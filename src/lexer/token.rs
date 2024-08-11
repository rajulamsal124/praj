#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum Token {
    // Single-character tokens
    LeftParen, RightParen,      // ( )
    LeftBrace, RightBrace,      // { }
    Comma, Dot, Minus, Plus,    // , . - +
    Semicolon, Slash, Star,     // ; / *

    // Literals
    Identifier(String),         // Variables, function names, etc.
    StringLiteral(String),      // String literals like "hello"
    Number(f64),                // Numeric literals like 123 or 4.56

    // Keywords
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    // End of file
    EOF,

    // Added to capture errors
    Error(String),              // To capture error messages
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Binary(Box<Expr>, BinOp, Box<Expr>), // Binary operation
    Literal(Literal),                    // Literal value (e.g., number, string)
    Variable(String),                    // Variable reference
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinOp {
    Add, // +
    Sub, // -
    Mul, // "*"
    Div, // /
}

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)] // Suppress dead code warnings for unused variants
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedToken,
    UnexpectedEOF,
}

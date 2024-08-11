mod lexer;

use lexer::lexer_core::Lexer;

fn main() {
    let source = "(3 + 42) * 7.5".to_string();
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}

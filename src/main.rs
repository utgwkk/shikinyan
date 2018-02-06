pub mod parser;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_lexer() {
    assert!(lexer::parse_Term("22").is_ok());
}

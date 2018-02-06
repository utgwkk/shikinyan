pub mod parser;
pub mod syntax;
pub mod eval;

use std::io::{stdin, stdout, Read, Write};

fn repl() {
    print!("# ");
    stdout().flush().ok();
    let mut input = String::new();
    stdin().read_to_string(&mut input);
    match parser::parse_Toplevel(input.as_str()) {
        Ok(parsed) => {
            match eval::evaluate(parsed) {
                Ok(result) => println!("{:?}", result),
                Err(err) => println!("error: {:?}", err)
            }
        },
        Err(e) => println!("error: {:?}", e)
    }
}

fn main() {
    loop {
        repl()
    }
}

#[test]
fn test_ml1() {
    assert!(parser::parse_Toplevel("1;;").is_ok());
    assert!(parser::parse_Toplevel("true;;").is_ok());
    assert!(parser::parse_Toplevel("false;;").is_ok());
    assert!(parser::parse_Toplevel("1 + 2;;").is_ok());
    assert!(parser::parse_Toplevel("1 + 2 * 3;;").is_ok());
    assert!(parser::parse_Toplevel("if 1 < 2 then 3 else 4;;").is_ok());
}

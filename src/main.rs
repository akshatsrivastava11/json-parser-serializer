mod json;
use json::*;
mod parse;
use parse::*;
mod token;
use token::*;

fn main() {
    let input = r#"{
        "name": "John Doe",
        "age": 30,
        "is_student": false,
        "courses": ["Math", "Science", "History"],
        "address": {
            "street": "123 Main St",
            "city": "Anytown"
        }
    }"#;
       let mut lexer = Lexer::new(input);

    println!("Input: {}", input);
    println!("Tokens:");

    for tok in lexer {
        match tok {
            Ok(t) => println!("  {:?}", t),
            Err(e) => {
                println!("Lexer error: {}", e);
                break;
            }
        }
    }
}

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

    match parse(input) {
        Ok(json_value) => {
            println!("\n--- Parsed successfully! (AST) ---");
            println!("{:?}", json_value);

            // 3. Test the Serializer (Round-Trip Test)
            let output_json = json_value.to_string();
            println!("\n--- Serialized output ---");
            println!("{}", output_json);
        }
        Err(e) => {
            eprintln!("\nPARSING FAILED: {:?}", e);
        }
    }
}

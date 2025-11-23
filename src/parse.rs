use std::{collections::HashMap, process::Command};

use crate::{
    json::JsonValue,
    token::{self, Lexer, Token},
};

#[derive(Debug)]
pub struct Parser {
    tokens: std::iter::Peekable<std::vec::IntoIter<Token>>,
}

pub type ParserResult<T> = Result<T, String>;
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        println!("--- Initializing Parser ---");
        Parser {
            tokens: tokens.into_iter().peekable(),
        }
    }
    pub fn consume(&mut self) -> Option<Token> {
        self.tokens.next()
    }
    pub fn peek(&mut self) -> Option<&Token> {
        self.tokens.peek()
    }
    fn expect(&mut self, expected: Token) -> ParserResult<()> {
        match self.consume() {
            Some(t) => {
                if t == expected {
                    Ok(())
                } else {
                    Err(format!("Expected {:?}, found {:?}", expected, t))
                }
            }
            _ => Err(format!("Unexpected token")),
        }
    }
    fn parse_value(&mut self) -> ParserResult<JsonValue> {
        let token = self.peek().ok_or("End of input");
        match token? {
            Token::OpenBrace => self.parse_object(),
            Token::OpenBracket => self.parse_array(),
            Token::LiteralNull => {
                self.consume();
                Ok(JsonValue::Null)
            }
            Token::LiteralTrue | Token::LiteralFalse => {
                let is_true = token.unwrap() == &Token::LiteralTrue;
                self.consume();
                Ok(JsonValue::Bool(is_true))
            }
            Token::Number(n) => {
                let value = JsonValue::Number(*n);
                self.consume(); // Consume the number token
                Ok(value)
            }
            Token::String(s) => {
                let value = JsonValue::String(s.clone());
                self.consume();
                Ok(value)
            }
            _ => Err(format!("Unexpected token: {:?}", token)),
        }
    }
    fn parse_object(&mut self) -> ParserResult<JsonValue> {
        self.expect(Token::OpenBrace)?;
        let mut map = HashMap::new();
        println!("--- Parsing Object --- , {:?}", self.tokens);
        while self.peek() != Some(&Token::CloseBrace) {
            if self.peek().is_none() {
                return Err("End of input".to_string());
            }
            let key = match self.consume() {
                Some(Token::String(s)) => s,
                _ => return Err("End of input".to_string()),
            };
            self.expect(Token::Colon)?;
            let value = self.parse_value()?;
            map.insert(key, value);
            match self.peek() {
                Some(&Token::Comma) => {
                    self.consume();
                }
                Some(&Token::CloseBrace) => {
                    break;
                }
                _ => {
                    return Err("Expected comma or closing brace".to_string());
                }
            }
        }
        self.expect(Token::CloseBrace)?;
        Ok(JsonValue::Object(map))
    }
    fn parse_array(&mut self) -> ParserResult<JsonValue> {
        self.expect(Token::OpenBracket)?;
        let mut arr = Vec::new();
        while self.peek() != Some(&Token::CloseBrace) {
            if self.peek().is_none() {
                return Err("End of input".to_string());
            }
            if self.peek() == Some(&Token::CloseBrace) {
                break;
            }
            let value = self.parse_value()?;
            arr.push(value);
            match self.peek() {
                Some(&Token::Comma) => {
                    self.consume();
                }
                Some(&Token::CloseBracket) => {
                    break;
                }
                Some(&Token::CloseBrace) => {
                    break;
                }
                _ => {
                    println!("self.peek(): {:?}", arr);
                    println!("--- Parsing Array Error --- , {:?}", self.peek());
                    return Err("Expected comma or closing bracket".to_string());
                }
            };
        }
        self.expect(Token::CloseBracket)?;
        Ok(JsonValue::Array(arr))
    }
}
pub fn parse(input: &str) -> ParserResult<JsonValue> {
    let tokens = match Lexer::new(input).collect() {
        Ok(toks) => toks,
        Err(e) => return Err(format!("Lexer error: {}", e)),
    };
    println!("Tokens collected for parsing: {:?}", tokens);
    let mut parser = Parser::new(tokens);
    // println!("--- Starting Parsing Process --- , {:?}",parser.tokens);
    let result = parser.parse_value();
    println!("Parser state after parsing: {:?}", result);
    if parser.consume().is_some() {
        return Err("Unexpected Token".to_string());
    }
    println!("Parsing completed. Result: {:#?}", result);
    result
}

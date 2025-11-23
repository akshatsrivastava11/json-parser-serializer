use core::str;
use std::{iter, path::Iter, string};

use crate::token;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Colon,
    Comma,
    String(String),
    Number(f64),
    LiteralTrue,
    LiteralFalse,
    LiteralNull,
}
pub type LexerResult<T> = Result<T, String>;

#[derive(Debug)]
pub struct Lexer<'a> {
    input: iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        println!("--- Initializing Lexer ---");
        Lexer {
            input: input.chars().peekable(),
        }
    }
    //consumes the next characher without peeking it
    fn consume(&mut self) -> Option<char> {
        self.input.next()
    }
    //peeks the next character without consuming it
    fn peek(&mut self) -> Option<char> {
        self.input.peek().copied()
    }

    //lexing a string token
    fn lex_string(&mut self) -> LexerResult<Token> {
        let mut string_content = String::new();
        loop {
            let next_char = self.consume().ok_or("Unexpected end of input")?;

            match next_char {
                '"' => return Ok(Token::String(string_content)),
                '\\' => {
                    let escaped_char = self.consume().ok_or("Unexpected end of input ")?;
                    match escaped_char {
                        '"' | '\\' | '/' => string_content.push(escaped_char),
                        'b' => string_content.push('\x08'),
                        'f' => string_content.push('\x0C'), // Form Feed
                        'n' => string_content.push('\n'),
                        'r' => string_content.push('\r'),
                        't' => string_content.push('\t'),
                        'u' => {
                            //implement a four digit unicode parsing
                            return Err(
                                "Unicode escape sequences are not supported yet".to_string()
                            );
                        }
                        _ => return Err(format!("Invalid escape sequence: \\{}", escaped_char)),
                    }
                }
                c => string_content.push(c),
            }
        }
    }

    //lexing a keyword token
    fn lex_keyword(&mut self, expected_tail: &str, token: Token) -> LexerResult<Token> {
        // dbg!("Lexing keyword: {:?} {:?}", expected_tail,self);
        for expected_char in expected_tail[1..].chars() {
            match self.consume() {
                Some(actual_char) if actual_char == expected_char => continue,
                _ => return Err("InvalidCharacter".to_string()),
            }
        }
        // dbg!("Lexed keyword: {:?}", token);
        Ok(token)
    }
    fn lex_number(&mut self, first_char: char) -> LexerResult<Token> {
        let mut number_str = String::new();
        number_str.push(first_char);
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() || c == '.' || c == 'e' || c == 'E' || c == '+' || c == '-' {
                number_str.push(self.consume().unwrap());
            } else {
                break;
            }
        }
        match number_str.parse::<f64>() {
            Ok(n) => Ok(Token::Number(n)),
            Err(_) => Err("Invalid numeber format".to_string()),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexerResult<Token>;
    fn next(&mut self) -> Option<Self::Item> {
        // println!("Lexer next called {:?}",self);
        //skipping whitespace

        while self.peek().map_or(false, |c| c.is_whitespace()) {
            self.consume();
        }
        //if the end of the input is reached
        let next_char = match self.consume() {
            Some(c) => c,
            None => return None,
        };
        let token = match next_char {
            '{' => Ok(Token::OpenBrace),
            '}' => Ok(Token::CloseBrace),
            '[' => Ok(Token::OpenBracket),
            ']' => Ok(Token::CloseBracket),
            ':' => Ok(Token::Colon),
            ',' => Ok(Token::Comma),
            '"' => self.lex_string(),
            c if c.is_ascii_digit() || c == '-' => self.lex_number(c),
            't' => self.lex_keyword("true", Token::LiteralTrue),
            'f' => self.lex_keyword("false", Token::LiteralFalse),
            'n' => self.lex_keyword("null", Token::LiteralNull),
            c => {
                dbg!("Lexed token: {:?}", c);

                println!("Invalid character encountered: {}", c);
                Err("InvalidCharacter".to_string() + ":" + &c.to_string())
            }
        };
        // dbg!("Lexed token: {:?} {:?}", token.iter().clone(),self);
        println!("Lexed token: {:?}", token);
        Some(token)
    }
}

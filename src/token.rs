use core::str;
use std::{iter, path::Iter, string};

use crate::token;

#[derive(Debug, PartialEq)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Colon,
    Comma,
    String(String),
    Number(f64),
}
pub type LexerResult<T> = Result<T, String>;

pub struct Lexer<'a> {
    input: iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
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
        let mut string_content=String::new();
        loop{
            let next_char=self.consume().ok_or("Unexpected end of input")?;
            match next_char {
                '"'=>return  Ok(Token::String(string_content)),
                '\\'=>{
                   let escaped_char=self.consume().ok_or("Unexpected end of input ")?;
                   match escaped_char{
                    '"' | '\\' | '/'=>string_content.push(escaped_char),
                    'b'=>string_content.push('\x08'),
                    'f' => string_content.push('\x0C'), // Form Feed
                    'n' => string_content.push('\n'),
                    'r' => string_content.push('\r'),
                    't' => string_content.push('\t'),
                    'u'=>{
                        //implement a four digit unicode parsing 
                        return  Err("Unicode escape sequences are not supported yet".to_string());
                    }
                    _=> return Err(format!("Invalid escape sequence: \\{}",escaped_char)),
                   }
                }
                c=>string_content.push(c),
            }
        }
    }

    //lexing a token
    // fn lex_keyword(&mut self,expected_tail:&str,token:Token)->LexerResult<Token>{
    //     for expected_char in expected_tail.chars(){
    //         match self.consume(){
                
    //         }
    //     }
    // }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = LexerResult<Token>;
    fn next(&mut self) -> Option<Self::Item> {
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
            _ => Err(format!("Unexpected character: {}", next_char)),
        };
        Some(token)
    }
}

use cursor::common::*;
use std::collections::VecDeque;
use std::str::FromStr;

use self::token::Token::*;
use self::token::*;

mod tests;
pub mod token;

struct Lexer<'a> {
    cursor: Cursor<'a>,
    input: &'a str,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Lexer {
            cursor: Cursor::new(input),
            input,
        }
    }

    pub fn tokenize(&mut self) -> VecDeque<Token> {
        let mut tokens = VecDeque::new();
        while !self.cursor.is_eof() {
            let token_kind = self.advance_token();
            if token_kind != WhiteSpace && token_kind != Comment {
                tokens.push_back(token_kind);
            }
        }
        tokens
    }

    fn advance_token(&mut self) -> Token {
        match self.cursor.next() {
            c if is_white_space(c) => {
                self.cursor.eat_whitespace();
                WhiteSpace
            }
            c if is_id_start(c) => self.identifier_or_keyword(),
            c if ";,@#$?{}[]()".contains(c) => {
                self.cursor.bump();
                Token::from_str(&c.to_string()).unwrap()
            }
            _ => {
                self.cursor.bump();
                Unknown
            }
        }
    }

    fn identifier_or_keyword(&mut self) -> Token {
        let len = self.cursor.eat_id();
        let str = self
            .input
            .get(self.cursor.eaten_len() - len..self.cursor.eaten_len())
            .unwrap();
        if let Ok(token) = Token::from_str(str) {
            token
        } else {
            Identifier(str.to_string())
        }
    }
}

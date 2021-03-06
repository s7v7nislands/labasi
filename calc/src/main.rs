use crate::Type::{EOF, INTEGER, PLUS};
use std::io::{stdin, stdout, Write};

#[derive(PartialEq, Clone, Debug)]
enum Type {
    INTEGER,
    PLUS,
    EOF,
}

#[derive(Debug)]
struct Token {
    typ: Type,
    value: String,
}

impl Clone for Token {
    fn clone(&self) -> Self {
        Token {
            typ: self.typ.clone(),
            value: self.value.clone(),
        }
    }
}

impl Default for Token {
    fn default() -> Self {
        Token {
            typ: EOF,
            value: String::from(""),
        }
    }
}

impl Token {
    fn new(typ: Type, value: String) -> Token {
        Token { typ, value }
    }
}

struct Interpreter {
    text: String,
    pos: usize,
    current_token: Token,
}

impl Interpreter {
    fn new(text: String) -> Interpreter {
        Interpreter {
            text,
            pos: 0,
            current_token: Token::default(),
        }
    }
    fn get_next_token(&mut self) -> Result<Token, String> {
        if self.pos > (self.text.len() - 1) {
            return Ok(Token::new(EOF, "".to_string()));
        }

        let current_char = self.text.chars().nth(self.pos).unwrap();

        if current_char.is_numeric() {
            self.pos += 1;
            return Ok(Token::new(INTEGER, current_char.to_string()));
        }

        if current_char.to_string() == "+".to_string() {
            self.pos += 1;
            return Ok(Token::new(PLUS, current_char.to_string()));
        }

        Err(format!("error parsing input: `{}`", current_char))
    }

    fn eat(&mut self, typ: Type) -> Result<(), String> {
        if self.current_token.typ == typ {
            match self.get_next_token() {
                Ok(token) => {
                    self.current_token = token;
                    return Ok(());
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
        return Err(format!("err token type: {:?}", typ));
    }

    fn expr(&mut self) -> Result<i64, String> {
        self.current_token = self.get_next_token()?;

        let left: i64 = self.current_token.value.parse().unwrap();
        self.eat(INTEGER)?;

        let _op = self.current_token.clone();
        self.eat(PLUS)?;

        let right: i64 = self.current_token.value.parse().unwrap();
        self.eat(INTEGER)?;

        let value = left + right;
        return Ok(value);
    }
}

fn main() {
    loop {
        print!("calc> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut interpreter = Interpreter::new(input.trim().to_string());
        match interpreter.expr() {
            Ok(value) => println!("{:?}", value),
            Err(error) => println!("error: {}", error),
        }

    }
}

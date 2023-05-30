use std::str::Chars;

use std::fmt::Display;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pos {
    file: String,
    column: usize,
    line: usize
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{:0>3}:{:0>3} -->\t", self.file, self.line, self.column)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Token {
    Comment(Pos, String),
    OpenParen(Pos),
    CloseParen(Pos),
    OpenBrackets(Pos),
    CloseBrackets(Pos),
    Identifier(Pos, String),
    Litteral(Pos, String),
    Equal(Pos),
    Star(Pos),
    Colon(Pos),
    Hat(Pos),
    At(Pos),
    Dot(Pos),
    Pipe(Pos),
    Other

}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Comment(a, b) => write!(f, "{}COMMENT[{:?}]", a, b),
            Token::OpenParen(a) => write!(f, "{} (", a),
            Token::CloseParen(a) => write!(f, "{} )", a),
            Token::OpenBrackets(a) => write!(f, "{} {{", a),
            Token::CloseBrackets(a) => write!(f, "{} }}", a),
            Token::Identifier(a, b) => write!(f, "{}ID[{:?}]", a, b),
            Token::Litteral(a, b) => write!(f, "{}LITT[{:?}]", a, b),
            Token::Equal(a) => write!(f, "{} =", a),
            Token::Star(a) => write!(f, "{} *", a),
            Token::Colon(a) => write!(f, "{} :", a),
            Token::Hat(a) => write!(f, "{} ^", a),
            Token::At(a) => write!(f, "{} @", a),
            Token::Dot(a) => write!(f, "{} .", a),
            Token::Pipe(a) => write!(f, "{} |", a),
            Token::Other => write!(f, "OTHER")
        }
    }
}

#[derive(Debug)]
pub struct Scanner<'a> {
    chars: Chars<'a>,
    loc: Pos
}



impl<'a> Scanner<'a> {

    pub fn new(s: &'a str, filename: &'a str) -> Self {

        Self {
            chars: s.chars(),
            loc: Pos{
                file: filename.to_string(),
                column: 1,
                line: 1
            }
        }
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {

        let mut output: Option<Token> = None; 

        if let Some(mut c) = self.chars.next() {

            while ['\n', '\t', ' ', '\r'].contains(&c) {

                if c == '\n' {
                    self.loc.line += 1;
                    self.loc.column = 1;
                }
                else {
                    self.loc.column += 1;
                }
                c = self.chars.next()?;
            }

            output = match c {

                '#' => {

                    let mut buffer = String::from(c);
                    let location = self.loc.clone();

                    for n in self.chars.by_ref().take_while(|x| x != &'\n') {
                        buffer.push(n);
                        self.loc.column += 1;
                    }

                    self.loc.line += 1;
                    self.loc.column = 1;

                    Some(Token::Comment(location, buffer))
                },
                c if c.is_alphanumeric() => {

                    let mut buffer = String::from(c);
                    let location = self.loc.clone();

                    for n in self.chars.by_ref().take_while(|x| x.is_alphanumeric()) {
                        buffer.push(n);
                        self.loc.column += 1;
                    }

                    self.loc.column += 1;

                    Some(Token::Identifier(location, buffer))
                },
                '\'' | '"' => {
                    let mut buffer = String::new();
                    let location = self.loc.clone();
                    let mut escaped = false;
                    let mut closed = false;

                    while let Some(n) = self.chars.by_ref().next() {

                        if escaped {
                            buffer.push(n);
                            escaped = false;
                        }
                        else if n == c {
                            closed = true;
                            self.loc.column += 1;
                            break;
                        }
                        else if n == '\\' {
                            escaped = true;
                        }
                        else {
                            buffer.push(n);
                        }
                        self.loc.column += 1;
                    }

                    if !closed {
                        panic!();
                    }

                    self.loc.column += 1;
                    
                    Some(Token::Litteral(location, buffer))
                },
                '{' => Some(Token::OpenBrackets(self.loc.clone())),
                '}' => Some(Token::CloseBrackets(self.loc.clone())),
                '(' => Some(Token::OpenParen(self.loc.clone())),
                ')' => Some(Token::CloseParen(self.loc.clone())),
                '=' => Some(Token::Equal(self.loc.clone())),
                '*' => Some(Token::Star(self.loc.clone())),
                ':' => Some(Token::Colon(self.loc.clone())),
                '^' => Some(Token::Hat(self.loc.clone())),
                '@' => Some(Token::At(self.loc.clone())),
                '.' => Some(Token::Dot(self.loc.clone())),
                '|' => Some(Token::Pipe(self.loc.clone())),
                _ =>  {
                    println!("{c:?}");
                    Some(Token::Other)
                }
            };
    
            self.loc.column += 1;
        }

            

        output

    }
}
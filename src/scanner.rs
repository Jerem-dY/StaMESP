use std::{str::Chars, error::Error};

use std::fmt::Display;
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub enum ScannerError{
    UnfinishedLitteral(Pos, String),
    UnknownEscape(Pos, String),
    UnknownToken(Pos, String)
}

impl Error for ScannerError {}

impl Display for ScannerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        match self {
            ScannerError::UnfinishedLitteral(loc, delimiter) => writeln!(f, "{}Unfinished string litteral with delimiter '{}'", loc, delimiter),
            ScannerError::UnknownEscape(loc, escaped) => writeln!(f, "{}Unknown escaped character '\\{}'", loc, escaped),
            ScannerError::UnknownToken(loc, token) => writeln!(f, "{}Undefined token '{}'", loc, token),
        }
        
    }
}

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
    Percent(Pos),
    Error

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
            Token::Percent(a) => write!(f, "{} %", a),
            Token::Error => write!(f, "ERROR")
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
    type Item = Result<Token, ScannerError>;

    fn next(&mut self) -> Option<Self::Item> {

        let mut output: Option<Result<Token, ScannerError>> = None; 

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

                    Some(Ok(Token::Comment(location, buffer)))
                },
                c if c.is_alphanumeric() => {

                    let mut buffer = String::from(c);
                    let location = self.loc.clone();

                    for n in self.chars.by_ref().take_while(|x| x.is_alphanumeric()) {
                        buffer.push(n);
                        self.loc.column += 1;
                    }

                    self.loc.column += 1;

                    Some(Ok(Token::Identifier(location, buffer)))
                },
                '\'' | '"' => {
                    let mut buffer = String::new();
                    let location = self.loc.clone();
                    let mut escaped = false;
                    let mut closed = false;

                    while let Some(n) = self.chars.by_ref().next() {

                        if escaped {

                            match n {
                                'n' => buffer.push('\n'),
                                't' => buffer.push('\t'),
                                'r' => buffer.push('\r'),
                                '\\' => buffer.push('\\'),
                                '"' => buffer.push('"'),
                                '\'' => buffer.push('\''),
                                _ => return Some(Err(ScannerError::UnknownEscape(location, n.to_string())))
                            }

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
                        return Some(Err(ScannerError::UnfinishedLitteral(location, c.to_string())))
                    }

                    self.loc.column += 1;
                    
                    Some(Ok(Token::Litteral(location, buffer)))
                },
                '{' => Some(Ok(Token::OpenBrackets(self.loc.clone()))),
                '}' => Some(Ok(Token::CloseBrackets(self.loc.clone()))),
                '(' => Some(Ok(Token::OpenParen(self.loc.clone()))),
                ')' => Some(Ok(Token::CloseParen(self.loc.clone()))),
                '=' => Some(Ok(Token::Equal(self.loc.clone()))),
                '*' => Some(Ok(Token::Star(self.loc.clone()))),
                ':' => Some(Ok(Token::Colon(self.loc.clone()))),
                '^' => Some(Ok(Token::Hat(self.loc.clone()))),
                '@' => Some(Ok(Token::At(self.loc.clone()))),
                '.' => Some(Ok(Token::Dot(self.loc.clone()))),
                '|' => Some(Ok(Token::Pipe(self.loc.clone()))),
                '%' => Some(Ok(Token::Percent(self.loc.clone()))),
                _ =>  {
                    Some(Err(ScannerError::UnknownToken(self.loc.clone(), c.to_string())))
                }
            };
    
            self.loc.column += 1;
        }

            

        output

    }
}
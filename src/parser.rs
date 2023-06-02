

use std::{collections::HashMap, fmt::Display, thread::current};

use num_derive::FromPrimitive;
use serde::{Serialize, Deserialize};

use super::scanner::*;
use std::error::Error;

#[derive(Debug)]
pub enum ParserError{
    DuplicateTransition(Pos, String),
    UndefinedIdentifier(Pos, String),
    Unclosed(Pos, String),
    NotAttached(Pos, String)
}

impl Error for ParserError {}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        match self {
            ParserError::DuplicateTransition(loc, transition) => writeln!(f, "{}Duplicate transition: '{}'", loc, transition),
            ParserError::UndefinedIdentifier(loc, id) => writeln!(f, "{}Undefined identifier: '{}'", loc, id),
            ParserError::Unclosed(loc, id) => writeln!(f, "{}Unclosed object: '{}'", loc, id),
            ParserError::NotAttached(loc, token) => writeln!(f, "{}Unattached object specifier: '{}'", loc, token),
        }
        
    }
}

#[derive(PartialEq, Debug)]
enum States {
    Void,
    Set,
    State,
    Transition
}

#[derive(Debug, Serialize, Deserialize, FromPrimitive, Clone, Copy, PartialEq)]
enum StateTypes {
    Through = 0b00000000,
    EntryPoint = 0b00000001,
    EndPoint = 0b00000010,
    Hub = 0b00000011
}

impl std::ops::BitOr for StateTypes {
    type Output = StateTypes;

    fn bitor(self, rhs: Self) -> Self::Output {

        let res:Option<StateTypes> = num::FromPrimitive::from_usize(self as usize | rhs as usize);

        if let Some(out) = res {
            out
        } else {
            todo!()
        }

    }
}

impl std::ops::BitAnd for StateTypes {
    type Output = StateTypes;

    fn bitand(self, rhs: Self) -> Self::Output {

        let res:Option<StateTypes> = num::FromPrimitive::from_usize(self as usize & rhs as usize);

        if let Some(out) = res {
            out
        } else {
            todo!()
        }

    }
}

impl std::ops::BitXor for StateTypes {
    type Output = StateTypes;

    fn bitxor(self, rhs: Self) -> Self::Output {

        let res:Option<StateTypes> = num::FromPrimitive::from_usize(self as usize ^ rhs as usize);

        if let Some(out) = res {
            out
        } else {
            todo!()
        }

    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
enum WritingBehaviour {
    WriteAfter,
    WriteBefore,
    NoWrite
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Object {
    values: Vec<usize>,
    id: String,
    transitions: Vec<(Pos, String, String, WritingBehaviour)>, // Mapping values to objects
    loc: Pos,
    t: StateTypes
}

#[derive(Debug)]
pub struct Symbols {
    pub objects: HashMap<String, Object>,
    values: Vec<(String, Pos)>,
    errors: Vec<Box<dyn Error>>
}

// We should use a hashmap in the making, but the symbol table should only use indices, and abstract away the names of the states (mostly)


impl Symbols {

    pub fn parser(scan: Scanner, verbose: bool) -> Self {

        let mut symbols = Self {
            objects: HashMap::new(),
            values: Vec::new(),
            errors: Vec::new(),
        };


        let mut state = vec![States::Void];
        let mut stack = Vec::<Token>::new();
        let mut writing_behaviour = WritingBehaviour::WriteAfter;
        let mut state_type = StateTypes::Through;
    
        let mut iter = scan.peekable();
        for token in iter.by_ref() {
            
            if let Ok(tok) = token {

                println!("{:?}", state.last());
                println!("{tok}");
    
                match &tok {
                    Token::Comment(_, _) => {},
                    Token::OpenParen(loc) => {

                        let top = stack.last();
                        match top {
                            Some(Token::Identifier(l, value)) => {
                                if !symbols.objects.contains_key(value) {
                                    symbols.objects.insert(value.clone(), Object {
                                        id: value.clone(), 
                                        loc: l.clone(), 
                                        values: Vec::new(), 
                                        transitions: Vec::new(), 
                                        t: state_type});
                                }
                                state_type = StateTypes::Through;
                                state.push(States::Set);
                            },
                            Some(_) => todo!(),
                            None => todo!(),
                        }
                    },
                    Token::CloseParen(loc) => {
                        if let Some(States::Set) = state.last() {
                            state.pop();
                            
                        }
                        else {
                            todo!()
                        }
                    },
                    Token::OpenBrackets(loc) => {
                        let top = stack.last();
                        match top {
                            Some(Token::Identifier(l, value)) => {
                                if !symbols.objects.contains_key(value) {
                                    symbols.objects.insert(value.clone(), Object {
                                        id: value.clone(), 
                                        loc: l.clone(), 
                                        values: Vec::new(), 
                                        transitions: Vec::new(), 
                                        t: state_type});
                                }
                                state_type = StateTypes::Through;
                                state.push(States::State);
                            },
                            Some(_) => todo!(),
                            None => todo!(),
                        }
                    },
                    Token::CloseBrackets(loc) => {
                        if let Some(States::State) = state.last() {
                            state.pop();
                        }
                        else {
                            todo!()
                        }
                    },
                    Token::Identifier(loc, value) => {
                        stack.push(tok);
                    },
                    Token::Litteral(loc, value) => {
                        if let Some(States::Set) = state.last() {

                            if let Some(Token::Identifier(_, id)) = stack.last() {
                                symbols.values.push((value.clone(), loc.clone()));
                                if let Some(obj) = symbols.objects.get_mut(id) {
                                    obj.values.push(symbols.values.len()-1);
                                }
                                else{
                                    todo!()
                                }
                            }
                            
                        }
                        else {
                            todo!()
                        }
                    },
                    Token::Equal(loc) => {
                        state.push(States::Transition);
                    },
                    Token::Star(loc) => {
                        if let Some(Token::Identifier(_, _)) = stack.last() {

                            if (state_type & StateTypes::EntryPoint) != StateTypes::Through {
                                symbols.errors.push(Box::new(ParserError::NotAttached(loc.clone(), format!("'*'")))); 
                            }
                            else {
                                state_type = state_type | StateTypes::EntryPoint;
                            }
                        }
                        else {
                            symbols.errors.push(Box::new(ParserError::NotAttached(loc.clone(), format!("'*'")))); 
                        }
                    },
                    Token::Colon(loc) => {
                        if let Some(Token::Identifier(_, _)) = stack.last() {

                            if (state_type & StateTypes::EndPoint) != StateTypes::Through {
                                symbols.errors.push(Box::new(ParserError::NotAttached(loc.clone(), format!("':'")))); 
                            }
                            else {
                                state_type = state_type | StateTypes::EndPoint;
                            }
                        }
                        else {
                            symbols.errors.push(Box::new(ParserError::NotAttached(loc.clone(), format!("':'")))); 
                        }
                    },
                    Token::Hat(loc) => {
                        if let Some(States::Transition) = state.last() {
                            if writing_behaviour != WritingBehaviour::WriteAfter {
                                symbols.errors.push(Box::new(ParserError::NotAttached(loc.clone(), format!("'^'")))); 
                            }
                            else {
                                writing_behaviour = WritingBehaviour::NoWrite;
                            }
                        }
                        else {
                            symbols.errors.push(Box::new(ParserError::NotAttached(loc.clone(), format!("'^'")))); 
                        }
                    },
                    Token::At(loc) => {
                        stack.push(Token::Identifier(loc.clone(), String::from('@')));
                    },
                    Token::Dot(loc) => {
                        if let Some(States::Transition) = state.last() {
                            stack.push(Token::Identifier(loc.clone(), String::from('.')));
                        }
                        else {
                            symbols.errors.push(Box::new(ParserError::NotAttached(loc.clone(), format!("'.'")))); 
                        }
                    },
                    Token::Pipe(loc) => todo!(),
                    Token::Percent(loc) => {
                        {
                            if let Some(States::Transition) = state.last() {
                                if writing_behaviour != WritingBehaviour::WriteAfter {
                                    symbols.errors.push(Box::new(ParserError::NotAttached(loc.clone(), format!("'%'")))); 
                                }
                                else {
                                    writing_behaviour = WritingBehaviour::WriteBefore;
                                }
                            }
                            else {
                                symbols.errors.push(Box::new(ParserError::NotAttached(loc.clone(), format!("'%'")))); 
                            }
                        }
                    },
                    Token::Error => todo!(),
                    Token::SemiColon(loc) => {

                        println!("{stack:?}");
                        if let Some(States::Transition) = state.last() {

                            if let (Some(Token::Identifier(_, target)), 
                                Some(Token::Identifier(loc_or, origin)),
                                Some(Token::Identifier(loc_cur, current_state)),
                                ) = (stack.pop(), stack.pop(), stack.last()) {

                                    if let Some(obj) = symbols.objects.get_mut(current_state) {
                                        for before in obj.transitions.iter().filter(|x| x.1 == origin) {
                                            // If there's a duplicate source
                                            
                                            symbols.errors.push(Box::new(ParserError::DuplicateTransition(loc_or.clone(), format!("{origin} => {target} ({}already defined here)", before.0)))); 
                                        }
                                        obj.transitions.push((loc_or, origin, target, writing_behaviour));
                                        state.pop();
                                        writing_behaviour = WritingBehaviour::WriteAfter;
                                    }
                                    else {
                                        //TODO: change error type?
                                        symbols.errors.push(Box::new(ParserError::UndefinedIdentifier(loc_cur.clone(), current_state.to_owned()))); 
                                    }
                                }

                        }
                        else {
                            stack.pop();
                        }
                    },
                }
            }
            else if verbose {
                symbols.errors.push(Box::new(token.unwrap_err()));
            }
        }

        if !stack.is_empty() {
            while let Some(token) = stack.pop() {
                match token {
                    Token::Identifier(loc, id) => symbols.errors.push(Box::new(ParserError::Unclosed(loc, id))),
                    _ => todo!()
                }
            }
        }

        for obj in symbols.objects.iter() {
            println!("{}", serde_json::to_string_pretty(&obj).unwrap());
        }

        for (i, val) in symbols.values.iter().enumerate() {
            println!("{i}: {val:?}");
        }

        for error in symbols.errors.iter() {
            print!("{error}");
        }

        symbols
    }

}

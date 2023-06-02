mod scanner;
mod parser;

use parser::*;
use std::fs;

#[cfg(test)]
mod tests {
    use crate::parser::Symbols;

    use super::scanner::*;
    use std::fs;

    #[test]
    fn it_works() {
        
        let input = fs::read_to_string("test.txt").expect("Cannot read the file");

        let scan = Scanner::new(input.as_str(), "test.txt");

        for tok in scan.clone() {
            match tok {
                Err(e) => println!("{}", e),
                Ok(val) => println!("{}", val)
            }
            
        }

        let sym = Symbols::parser(scan, true);

        

        
    }
}

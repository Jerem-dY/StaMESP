mod scanner;

use scanner::*;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
        let input = fs::read_to_string("test.txt").expect("Cannot read the file");

        let scan = Scanner::new(input.as_str(), "test.txt");

        for tok in scan {
            println!("{}", tok);
        }
    }
}

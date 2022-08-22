use std::fmt;
use std::fs;

pub mod Player; //Generates a sudoku board with random numbers



impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Our Name is: {}", self.name)
    }
}

pub fn printF(){
    //Prints the file that is read
    let results: Player =readF();
    println!("{}",results.to_string());
}
pub fn readF() -> String {
    /**
    @return: File contents as a string
    **/
    let contents: String = fs::read_to_string("./data/data.yaml")
        .expect("Something went wrong reading the file");
    return contents;
}
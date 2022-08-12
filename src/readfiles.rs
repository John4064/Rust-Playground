
use std::fs;


pub fn printF(){
    //Prints the file that is read
    let results =readF();
    println!("{}",results);
}
pub fn readF() -> String {
    /**
    @return: File contents as a string
    **/
    let contents: String = fs::read_to_string("./data/data.yaml")
        .expect("Something went wrong reading the file");
    return contents;
}

use std::env;
use std::fs;
pub fn printF(){
    //Prints the file that is read

    readF();
}
pub fn readF(){
    let contents = fs::read_to_string("data.yaml")
        .expect("Something went wrong reading the file");
}
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
/**@Author John Parkhurst

 * @brief learning about Rust
 */

pub mod examples;//examples::run_code(); example code

pub mod sudokugen; //Generates a sudoku board with random numbers

pub mod readfiles; //Read/write files in rust


fn main() {
    sudokugen::generate()
}

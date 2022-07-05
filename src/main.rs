#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
/**@Author John Parkhurst

 * @brief learning about Rust
 */

pub mod examples;//examples::run_code(); example code
//let mut to create a mutable variable
pub mod tempconvert;




fn main() {
    println!("{}",tempconvert::c_to_f(-50.0));

}

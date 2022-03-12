/**@Author John Parkhurst
 * @brief learning about Rust
 */

//let mut to create a mutable variable

fn example_type_cast(value: i32){
    let mut my_base = value;
    my_base+=5;
    let my_extra = 3.4;
    let test = my_base as f64 +my_extra;
    println!("{}", test);
}

fn example_array(){
    let my_arr = [3,4,3,32];
    println!("{:?}", my_arr);
}

fn main() {
    println!("a");
}

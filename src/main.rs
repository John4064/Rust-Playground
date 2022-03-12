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

fn example_loops(){
    //..is exclusive 0-5 ..= is inclusive 0-5
    println!("Exclusive:");
    for x in 0..5 {
        println!("{}", x);
    }
   println!("Inclusive:");
    for x in 0..=5 {
        println!("{}", x);
    }
    println!("Returning a value within a loop!");
    let mut x = 0;
    let v = loop {
        x += 1;
        if x == 13 {
            break "found the 13";
        }
    };
    println!("from loop: {}", v);
    return;
}

fn example_conditional(){
    
}
fn main() {
    example_loops();

}

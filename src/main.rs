/**@Author John Parkhurst
 * @brief learning about Rust
 */

//let mut to create a mutable variable



fn example_type_cast(value: i32) {
    let mut my_base = value;
    my_base += 5;
    let my_extra = 3.4;
    let test = my_base as f64 + my_extra;
    println!("{}", test);
}

fn example_array() {
    let my_arr = [3, 4, 3, 32];
    println!("{:?}", my_arr);
}

fn example_loops() {
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

fn example_conditional() {
    let y = 5;
    if y > 4 {
        println!("HI");
    }
}
/**
Below will be some Leetcode problems!
**/
//https://leetcode.com/problems/concatenation-of-array/
fn get_concatenation(nums: Vec<i32>) -> Vec<i32>{
    let mut ans_arr = vec![];
    let size = nums.len();
    for x in 0..size*2 {
        ans_arr.push(nums[x%size]);
    }
    return ans_arr;
}
//https://leetcode.com/problems/build-array-from-permutation/
fn build_array(nums: Vec<i32>) -> Vec<i32> {
    let mut ans_arr = vec![];
    for x in 0..nums.len(){
        ans_arr.push(nums[nums[x]as usize]);
    }
    return ans_arr;
}
//https://leetcode.com/problems/running-sum-of-1d-array/
fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut ans_arr = vec![];
    let mut curr_sum=0;
    for x in 0..nums.len(){
        curr_sum+=nums[x];
        ans_arr.push(curr_sum);
    }
    return ans_arr;
}


fn main() {
    if true {
        println!("{:?}", running_sum( vec![0,1,2,3,4,5,6,7,8,9,-5]));
    }else{
        //Examples
        example_conditional();
        example_loops();
        example_array();
        example_type_cast(3);
        println!("{:?}", get_concatenation( vec![1,2,1]));
        println!("{:?}", build_array( vec![0,2,1,5,3,4]));
        println!("{:?}", running_sum( vec![5,0,1,2,3,4]));

    }
}

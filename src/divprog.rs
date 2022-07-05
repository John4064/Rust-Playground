pub fn divisor(num: i32)-> Vec<i32>{
    let mut ans_arr = vec![];

    for i in 1..num+1{
        if num%i==0{
            ans_arr.push(i);
        }
    }
    return ans_arr;
}
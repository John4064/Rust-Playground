
pub fn factorial(num: i32)-> i32{
    let mut answer = 1;
    for x in 1..num+1{
        answer*=x;
    }
    return answer;
}
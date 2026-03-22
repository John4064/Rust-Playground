

pub fn c_to_f(num: f32)-> f32{
    let answer=(num*1.8)+32.0;
    return answer;
}

pub fn f_to_c(num: f32)-> f32{
    let answer= (num-32.0)*0.5556;
    return answer;
}
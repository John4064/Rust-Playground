use rand::Rng;
const BOARDSIZE:usize = 9;


fn printout(state :&[[u8; BOARDSIZE];9]){
    for x in 0..BOARDSIZE{
        for y in 0..BOARDSIZE{
            print!("{}",state[x][y]);
        }
        println!("");
    }
}
//This will generate sudoko board
pub fn generate(){
    //4 by 4 matrix
    let mut rng = rand::thread_rng();
    let mut state = [[0u8; BOARDSIZE];9];
    let mut temp_string: String = "123456789".to_string();
    let mut used_nums: String ="".to_string();
    for i in 0..BOARDSIZE{

        if true {
            let mut temp_num: u8 = rng.gen_range(1, 1 + BOARDSIZE as u8);
            while !temp_string.contains(&temp_num.to_string()) && used_nums.contains(&temp_num.to_string()) {
                temp_num = rng.gen_range(1, 1 + BOARDSIZE as u8);
            }
            temp_string = temp_string.replace(&temp_num.to_string(), "");
            used_nums.push_str(temp_num.to_string().trim());
            println!("{}", temp_string);
            //now we auto
            state[i][rng.gen_range(0, BOARDSIZE as usize)] = temp_num;
        }
    }
    printout(&state);
    //println!("{}",state[3][3]);
}
use rand::Rng;
const BOARDSIZE:usize = 9;


fn printout(){

}
pub fn generate(){
    //4 by 4 matrix
    let mut rng = rand::thread_rng();
    let mut state = [[0u8; BOARDSIZE];9];
    for i in 0..BOARDSIZE{
        let mut tempString : String = "".to_string();
        for j in 0..BOARDSIZE {
            let mut tempnum: u8 =  rng.gen_range(1, 1+BOARDSIZE as u8);
            while tempString.contains(&tempnum.to_string()) {
                tempnum =  rng.gen_range(1, 1+BOARDSIZE as u8);
            }
            tempString.push_str(tempnum.to_string().trim());
            state[i][j] = tempnum;
            print!("{} ", state[i][j]);
        }
        println!("");
    }

    //println!("{}",state[3][3]);
}
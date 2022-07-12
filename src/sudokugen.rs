use rand::Rng;
const BOARDSIZE:usize = 9;


fn printout(){

}
//This will generate sudoko board
pub fn generate(){
    //4 by 4 matrix
    let mut rng = rand::thread_rng();
    let mut state = [[0u8; BOARDSIZE];9];
    let mut tempString : String = "123456789".to_string();
    for i in 0..BOARDSIZE{
        let mut boolexp = false;
        for j in 0..BOARDSIZE {
            if boolexp == false {
                let mut tempnum: u8 =  rng.gen_range(1, 1+BOARDSIZE as u8);

                 while tempString.contains(&tempnum.to_string()) {
                     tempnum =  rng.gen_range(1, 1+BOARDSIZE as u8);
                 }
                state[i][j] = tempnum;
                print!("{} ", state[i][j]);
                boolexp = true;
            }else{
                state[i][j] =0;
                print!("{} ", state[i][j]);
            }

        }
        println!("");
    }

    //println!("{}",state[3][3]);
}
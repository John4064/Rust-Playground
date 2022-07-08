use rand::Rng;
const BOARDSIZE:usize = 9;


fn printout(){

}
pub fn generate(){
    //4 by 4 matrix
    let mut rng = rand::thread_rng();
    let mut state = [[0u8; BOARDSIZE];9];
    for i in 0..BOARDSIZE{
        for j in 0..BOARDSIZE {
            let tempnum: u8 =  rng.gen_range(0, BOARDSIZE as u8);
            state[i][j] = tempnum;
            print!("{} ", state[i][j]);
            if tempnum[i].contains(){

            }
        }
        println!("");
    }

    //println!("{}",state[3][3]);
}
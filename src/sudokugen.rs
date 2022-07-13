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
    let mut tempString : String = "123456789".to_string();
    let mut usedNums : String ="".to_string();
    for i in 0..BOARDSIZE{
        let mut boolexp = false;
        for j in 0..BOARDSIZE {
            if boolexp == false {
                let mut tempnum: u8 =  rng.gen_range(1, 1+BOARDSIZE as u8);
                 while !tempString.contains(&tempnum.to_string())  && usedNums.contains(&tempnum.to_string()) {
                     tempnum = rng.gen_range(1, 1+BOARDSIZE as u8);
                 }
                tempString=tempString.replace(&tempnum.to_string(),"");
                usedNums.push_str(tempnum.to_string().trim());
                println!("{}",tempString);
                //now we auto
                state[i][j] = tempnum;

                boolexp = true;
            }else{
                state[i][j] =0;
            }

        }
    }
    printout(&state);
    //println!("{}",state[3][3]);
}
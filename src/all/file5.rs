extern crate rand;
use rand::Rng;

pub fn rndgen() {
    let mut newv: Vec<i32>  = Vec::new();
    let rndNum = rand::thread_rng().gen_range(1..100);
    if newv.len() == 0{
        newv.push(rndNum);
    }
    else{
        for i in 1..100 {
            if rndNum != newv[i]{
            newv.push(rndNum);
            } continue;
        }
    }
    //newv.push(rndNum);
    let ln = newv.len();
    println!("length is {}", ln);
    for j in newv {
        println!("{}", j);
    }
    
        
        // if rndNum != {
        //     newv.append();
        //continue;



}
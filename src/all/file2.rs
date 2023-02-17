//use std::io;

struct Details{
    name: String,
    age: u8
}

trait IsEligible {
    fn eligible(&self);
    fn not_eligible(&self);
}

impl IsEligible for Details {
    fn eligible(&self) {
        if self.age < 18 {
            println!("Not eligible");
        } println!("Eligible");
    }

    fn not_eligible(&self) {
        println!("{}'s age is {}.",self.name,self.age);
    }

}

pub fn newfun(){
    let ins = Details{
        name: String::from("Sumit"),
        age: 18
    };
    ins.not_eligible();
    ins.eligible();
}
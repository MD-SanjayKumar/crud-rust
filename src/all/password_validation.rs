use std::io;
extern crate regex;
use regex::Regex;

pub fn check_validity(){
    let mut value = String::new();
    //let rex = Regex::new(r"[A-Z][a-z][0-9][@_.!-*]").unwrap();
    let rex1 = Regex::new(r"[A-Z]").unwrap();
    let rex2 = Regex::new(r"[a-z]").unwrap();
    let rex3 = Regex::new(r"[0-9]").unwrap();
    let rex4 = Regex::new(r"[_@#]").unwrap();
    let rex5 = Regex::new(r"\A[_0-9]").unwrap();
    println!("Enter a password :");
    io::stdin().read_line(&mut value).expect("value not found.");
    let val = value.trim();
    if val.len() <= 7 {
        println!("Minimum 8 Character required.");
    }
    else if val.len() >= 26 {
        println!("Can't exceed maximum limit. (Should be less than 26 char.)");
    }
    else {
    if rex5.is_match(val) == true {
        println!("First character shouldn't be '_' or Numeric value.");
    }
    else{
    if rex1.is_match(val) == true && rex2.is_match(val) == true && rex3.is_match(val) == true && rex4.is_match(val) == true{
        println!("Valid ; Password Strength > Strong.");

    }
    else if rex1.is_match(val) == false && rex2.is_match(val) == true && rex3.is_match(val) == true && rex4.is_match(val) == true ||
    rex1.is_match(val) == true && rex2.is_match(val) == false && rex3.is_match(val) == true && rex4.is_match(val) == true ||
    rex1.is_match(val) == true && rex2.is_match(val) == true && rex3.is_match(val) == false && rex4.is_match(val) == true ||
    rex1.is_match(val) == true && rex2.is_match(val) == true && rex3.is_match(val) == true && rex4.is_match(val) == false{
        println!("Valid ; Password Strength > Moderate");
    }
    else if rex1.is_match(val) == false && rex2.is_match(val) == false && rex3.is_match(val) == true && rex4.is_match(val) == true ||
    rex1.is_match(val) == true && rex2.is_match(val) == false && rex3.is_match(val) == false && rex4.is_match(val) == true ||
    rex1.is_match(val) == true && rex2.is_match(val) == true && rex3.is_match(val) == false && rex4.is_match(val) == false ||
    rex1.is_match(val) == false && rex2.is_match(val) == true && rex3.is_match(val) == true && rex4.is_match(val) == false{
        println!("Valid ; Password Strength > Weak");
    }
    else{
        println!("Password is too weak. Retry!");   
    }
    }
}

}
mod password_validation;
use std::io;
extern crate regex;
use regex::Regex;

pub fn check_username(){
    let mut valv = String::new();
    //let rex = Regex::new(r"[A-Z][a-z][0-9][@_.!-*]").unwrap();
    let rex1 = Regex::new(r"[A-Z]").unwrap();
    let rex2 = Regex::new(r"[a-z]").unwrap();
    let rex3 = Regex::new(r"[0-9]").unwrap();
    let rex4 = Regex::new(r"[_.-]").unwrap();
    let rex5 = Regex::new(r"\A[_0-9]").unwrap();
    println!("Enter username :");
    io::stdin().read_line(&mut valv).expect("value not found.");
    let val1 = valv.trim();
    if val1.len() <= 5 {
        println!("Minimum 6 character required.");
    }
    else if val1.len() >= 16 {
        println!("Can't exceed maximaum character limit.\n It should be less than 16 characters.");
    }
    else {
        if rex5.is_match(val1) == true {
            println!("First character shouldn't be '_' or Numeric value.");
        }
        else{
        
        if rex1.is_match(val1) == true && rex2.is_match(val1) == true && (rex3.is_match(val1) == true || rex4.is_match(val1) == true)
        {
            password_validation::check_validity();            
        }
        else {
            println!("Oops! Invalid");
        }
        }
    }
    
    }

use std::collections::HashMap;
use std::io;
extern crate regex;
use regex::Regex;

use crate::usr_details;

static mut nm: String = String::new();
static mut X: String = String::new();
static mut Y: String = String::new();
static mut add: String = String::new();

pub fn main_op(users: &mut HashMap<String, usr_details>) {
    check_username();
    unsafe {
        let details = usr_details {
            name: nm.to_string(),
            password: Y.to_string(),
            username: X.to_string(),
            address: add.to_string(),
        };
        users.insert(X.to_string(), details);
    }
}

//User
fn check_username() {
    let mut valv = String::new();
    //let rex = Regex::new(r"[A-Z][a-z][0-9][@_.!-*]").unwrap();
    let rex1 = Regex::new(r"[A-Z]").unwrap();
    let rex2 = Regex::new(r"[a-z]").unwrap();
    let rex3 = Regex::new(r"[0-9]").unwrap();
    let rex4 = Regex::new(r"[_.-]").unwrap();
    let rex5 = Regex::new(r"\A[_0-9]").unwrap();
    println!("Enter your name :");
    unsafe {
        io::stdin().read_line(&mut nm).expect("value not found.");
    }
    println!("Enter a username :");
    io::stdin().read_line(&mut valv).expect("value not found.");
    let val1 = valv.trim();
    if val1.len() <= 5 {
        println!("Minimum 6 character required.");
    } else if val1.len() >= 16 {
        println!("Can't exceed maximaum character limit.\n It should be less than 16 characters.");
    } else {
        if rex5.is_match(val1) == true {
            println!("First character shouldn't be '_' or Numeric value.");
        } else {
            if rex1.is_match(val1) == true
                && rex2.is_match(val1) == true
                && (rex3.is_match(val1) == true || rex4.is_match(val1) == true)
            {
                check_validity();
                unsafe {
                    X = val1.to_string();
                }
            } else {
                println!("Oops! Invalid");
            }
        }
    }
}
//Pass
fn check_validity() {
    let mut value = String::new();
    //let rex = Regex::new(r"[A-Z][a-z][0-9][@_.!-*]").unwrap();
    let rex1 = Regex::new(r"[A-Z]").unwrap();
    let rex2 = Regex::new(r"[a-z]").unwrap();
    let rex3 = Regex::new(r"[0-9]").unwrap();
    let rex4 = Regex::new(r"[_@#]").unwrap();
    let rex5 = Regex::new(r"\A[_0-9]").unwrap();
    println!("Enter address :");
    unsafe {
        io::stdin().read_line(&mut add).expect("value not found.");
    }
    println!("Enter a password :");
    io::stdin().read_line(&mut value).expect("value not found.");
    let val = value.trim();
    if val.len() <= 7 {
        println!("Minimum 8 Character required.");
    } else if val.len() >= 26 {
        println!("Can't exceed maximum limit. (Should be less than 26 char.)");
    } else {
        if rex5.is_match(val) == true {
            println!("First character shouldn't be '_' or Numeric value.");
        } else {
            if rex1.is_match(val) == true
                && rex2.is_match(val) == true
                && rex3.is_match(val) == true
                && rex4.is_match(val) == true
            {
                println!("Valid;\nPassword Strength > Strong.");
                unsafe {
                    Y = val.to_string();
                }
            } else if rex1.is_match(val) == false
                && rex2.is_match(val) == true
                && rex3.is_match(val) == true
                && rex4.is_match(val) == true
                || rex1.is_match(val) == true
                    && rex2.is_match(val) == false
                    && rex3.is_match(val) == true
                    && rex4.is_match(val) == true
                || rex1.is_match(val) == true
                    && rex2.is_match(val) == true
                    && rex3.is_match(val) == false
                    && rex4.is_match(val) == true
                || rex1.is_match(val) == true
                    && rex2.is_match(val) == true
                    && rex3.is_match(val) == true
                    && rex4.is_match(val) == false
            {
                println!("Valid;\nPassword Strength > Moderate");
                unsafe {
                    Y = val.to_string();
                }
            } else if rex1.is_match(val) == false
                && rex2.is_match(val) == false
                && rex3.is_match(val) == true
                && rex4.is_match(val) == true
                || rex1.is_match(val) == true
                    && rex2.is_match(val) == false
                    && rex3.is_match(val) == false
                    && rex4.is_match(val) == true
                || rex1.is_match(val) == true
                    && rex2.is_match(val) == true
                    && rex3.is_match(val) == false
                    && rex4.is_match(val) == false
                || rex1.is_match(val) == false
                    && rex2.is_match(val) == true
                    && rex3.is_match(val) == true
                    && rex4.is_match(val) == false
            {
                println!("Retry!\nPassword Strength > Weak");
            } else {
                println!("Try again.");
            }
        }
    }
}

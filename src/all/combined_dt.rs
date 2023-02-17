use crate::is_logged;
use crate::usr_details;
use std::collections::HashMap;
use std::io;

pub fn check_login(users: &mut HashMap<String, usr_details>) {
    if unsafe { is_logged == true } {
        println!("Already logged in.");
    } else {
        let mut vx = String::new();
        let mut vy = String::new();
        println!("Enter username:");
        io::stdin().read_line(&mut vx).expect("not found");
        match users.get(&vx.trim().to_string()) {
            Some(cf) => {
                println!("Enter password:");
                io::stdin().read_line(&mut vy).expect("not found");
                if cf.password.trim() == vy.trim() {
                    unsafe {
                        is_logged = true;
                    }
                    println!("\nWelcome {}", cf.name);
                } else {
                    println!("Please check input.");
                }
            }
            None => println!("Please check input."),
        }
    }
}

pub fn check_logout() {
    if unsafe { is_logged == false } {
        println!("Already logged out.");
    } else {
        unsafe {
            is_logged = false;
        }
        println!("Logout successfully.");
    }
}

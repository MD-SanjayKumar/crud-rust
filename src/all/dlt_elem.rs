use crate::is_logged;
use crate::usr_details;
use std::collections::HashMap;
use std::io;

pub fn remove_element(users: &mut HashMap<String, usr_details>) {
    if unsafe { is_logged == false } {
        println!("Please login first.");
    } else {
        let mut vx = String::new();
        let mut vy = String::new();
        println!("Confirm username:");
        io::stdin().read_line(&mut vx).expect("not found");
        println!("Confirm password:");
        io::stdin().read_line(&mut vy).expect("not found");
        match users.get(&vx.trim().to_string()) {
            Some(cf) => {
                if vy.trim() == cf.password.trim() {
                    users.remove(&vx.trim().to_string());
                    unsafe {
                        is_logged = false;
                    }
                    println!("Deleted successfully.");
                } else {
                    println!("Please check input.");
                }
            }
            None => println!("Please check input."),
        }
    }
}

pub fn update_element(users: &mut HashMap<String, usr_details>) {
    if unsafe { is_logged == false } {
        println!("Please login first.");
    } else {
        let mut vx = String::new();
        let mut vy = String::new();
        let mut vy1 = String::new();
        println!("Confirm username:");
        io::stdin().read_line(&mut vx).expect("not found");
        println!("Confirm password:");
        io::stdin().read_line(&mut vy).expect("not found");
        println!("Enter new address:");
        io::stdin().read_line(&mut vy1).expect("not found");
        match users.get(&vx.trim().to_string()) {
            Some(cf) => {
                if vy.trim() == cf.password.trim() {
                    (users.get_mut(&vx.trim().to_string()).unwrap()).address = vy1.to_string();
                    println!("Updated successfully.");
                } else {
                    println!("Please check input.");
                }
            }
            None => println!("Please check input."),
        }
    }
}

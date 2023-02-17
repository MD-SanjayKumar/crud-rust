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
        // println!("Enter new address:");
        // io::stdin().read_line(&mut vy1).expect("not found");
        let mut can_change: bool = true;
        match users.get(&vx.trim().to_string()) {
            Some(cf) => {
                if vy.trim() == cf.password.trim() {
                    while can_change {
                        let mut v = String::new();
                        println!("- - - - - - - - - -\nChoose option :\n1.Change name\n2.Change address\n- - - - - - - - - -\nSelect one option:");
                        io::stdin().read_line(&mut v).expect("Value not found");
                        match v.as_str().trim() {
                            "1" | "name" => {
                                println!("Enter new name:");
                                io::stdin().read_line(&mut vy1).expect("not found");
                                (users.get_mut(&vx.trim().to_string()).unwrap()).name =
                                    vy1.to_string();
                                println!("Name updated successfully.");
                                can_change = false;
                            }
                            "2" | "address" => {
                                println!("Enter new address:");
                                io::stdin().read_line(&mut vy1).expect("not found");
                                (users.get_mut(&vx.trim().to_string()).unwrap()).address =
                                    vy1.to_string();
                                println!("Address updated successfully.");
                                can_change = false;
                            }
                            _ => println!("No option selsected."),
                        }
                    }
                    // (users.get_mut(&vx.trim().to_string()).unwrap()).address = vy1.to_string();
                    // println!("Updated successfully.");
                } else {
                    println!("Please check input.");
                }
            }
            None => println!("Please check input."),
        }
    }
}

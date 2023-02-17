use crate::is_admin_logged;
use crate::is_logged;
use crate::usr_details;
use std::collections::HashMap;
use std::io;

pub fn check_users(adm: &mut HashMap<String, String>, usr: &mut HashMap<String, usr_details>) {
    println!("List of users:");
    if usr.is_empty() {
        println!("No user found.");
    } else {
        for (x, y) in usr.iter() {
            println!("> {}", x);
        }
    }
}

pub fn dlt_users(adm: &mut HashMap<String, String>, usr: &mut HashMap<String, usr_details>) {
    let mut vx = String::new();
    println!("Enter the username of target user:");
    io::stdin().read_line(&mut vx).expect("not found");
    match usr.get(&vx.trim().to_string()) {
        Some(cm) => {
            if vx.as_str().trim() == cm.username.trim() {
                usr.remove(&vx.trim().to_string());
                unsafe {
                    is_logged = false;
                }
                println!("Deleted successfully.");
            }
        }
        None => println!("Please check input."),
    }
}

pub fn check_alogout() {
    if unsafe { is_admin_logged == false } {
        println!("Already logged out.");
    } else {
        unsafe {
            is_admin_logged = false;
        }
        println!("Logout & exit successfully.");
    }
}

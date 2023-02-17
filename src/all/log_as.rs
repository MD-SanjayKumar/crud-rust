use crate::is_admin_logged;
use crate::usr_details;
use std::collections::HashMap;
use std::io;
mod ad_log;
//#[path = "all/combined_up.rs"]
//mod combined_up;
//#[path = "all/dlt_elem.rs"]
//mod dlt_elem;

pub fn admin(adm: &mut HashMap<String, String>, usr: &mut HashMap<String, usr_details>) {
    let mut ip = true;
    let mut id = String::new().to_string();
    let mut pass = String::new();
    println!("Enter admin username :");
    io::stdin().read_line(&mut id).expect("Value not found");
    println!("Enter admin password :");
    io::stdin().read_line(&mut pass).expect("Value not found");
    match adm.get(&id.trim().to_string()) {
        Some(v) => {
            if pass.as_str().trim() == v.as_str().trim() {
                unsafe {
                    is_admin_logged = true;
                }
                while ip {
                    let mut dso = String::new();
                    println!("- - - - - - - - - -\n1.View users\n2.Delete user\n3.Logout(exit)\n- - - - - - - - - -\nSelect one option:");
                    io::stdin().read_line(&mut dso).expect("Value not found");
                    match dso.as_str().trim() {
                        "1" | "view" => ad_log::check_users(adm, usr),
                        "2" | "delete" => ad_log::dlt_users(adm, usr),
                        "3" | "exit" => {
                            ad_log::check_alogout();
                            ip = false;
                        }
                        _ => println!("No option selected^^^"),
                    }
                }
            }
        }
        None => println!("Please check input"),
    }
}

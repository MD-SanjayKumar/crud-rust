use crate::combined_dt::check_logout;
use std::{collections::HashMap, io};
#[path = "all/combined_dt.rs"]
mod combined_dt;
#[path = "all/combined_up.rs"]
mod combined_up;
#[path = "all/dlt_elem.rs"]
mod dlt_elem;
#[path = "all/log_as.rs"]
mod log_as;

pub struct usr_details {
    name: String,
    username: String,
    password: String,
    address: String,
}
static mut is_admin_logged: bool = false;
static mut is_logged: bool = false;
fn main() {
    let mut adata: HashMap<String, String> = HashMap::new();
    let mut udata: HashMap<String, usr_details> = HashMap::new();
    adata.insert("Admin@123".to_string(), "Admin".to_string());
    let mut iq = true;
    while iq {
        let mut v = String::new();
        println!("- - - - - - - - - -\n1.Register\n2.Login\n3.Update\n4.Delete\n5.Logout\n6.Exit\n- - - - - - - - - -\n7.Login as ADMIN\n- - - - - - - - - -\nSelect one option:");
        io::stdin().read_line(&mut v).expect("Value not found");
        match v.as_str().trim() {
            "1" | "register" => combined_up::main_op(&mut udata),
            "2" | "login" => combined_dt::check_login(&mut udata),
            "3" | "update" => dlt_elem::update_element(&mut udata),
            "4" | "delete" => dlt_elem::remove_element(&mut udata),
            "5" | "logout" => combined_dt::check_logout(),
            "6" | "exit" => {
                iq = false;
                println!("Exit :)")
            }
            "7" | "admin" => log_as::admin(&mut adata, &mut udata),
            _ => println!("No option selected^^^"),
        }
    }
}

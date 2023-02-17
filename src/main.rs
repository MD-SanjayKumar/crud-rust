use std::{io, collections::HashMap};

use crate::combined_dt::check_logout;
//#[path = "all/file1.rs"] mod file1;
//#[path = "all/file2.rs"] mod file2;
//#[path = "all/file3.rs"] mod file3;
//#[path = "all/file4.rs"] mod file4;
//#[path = "all/file5.rs"] mod file5;
//#[path = "all/username_validation.rs"] mod username_validation; 
//#[path = "all/password_validation.rs"] mod password_validation;
#[path = "all/combined_up.rs"] mod combined_up;
#[path = "all/combined_dt.rs"] mod combined_dt;
#[path = "all/dlt_elem.rs"] mod dlt_elem;

pub struct usr_details{
  name:String,
  username: String,
  password: String,
  address: String
}
static mut is_logged: bool = false;
fn main(){
  let mut data: HashMap<String,usr_details>= HashMap::new();
  
  let mut iq = true;
  while iq {
    let mut v = String::new(); 
    println!("---------------\n1.Register(r)\n2.Login(l)\n3.Update(u)\n4.Delete(d)\n5.Logout(o)\n6.Exit(e)\n---------------");
  io::stdin().read_line(&mut v).expect("Value not found");
  match v.as_str().trim(){
      "r" | "register" => combined_up::main_op(&mut data),
      "l"|"login" => combined_dt::check_login(&mut data),
      "u" | "update" => dlt_elem::update_element(&mut data),
      "d" | "delete" => dlt_elem::remove_element(&mut data),
      "o" | "logout" => combined_dt::check_logout(),
      "e" | "exit" => { iq = false;
                        println!("Exit :)")},
      _ => println!("No option selected^")
  }
}
}

  //  file1::nfq();
  //  file2::newfun();
  //  file3::switch();
  //  file4::newhash();
  //  file5::rndgen();
  //  username_validation::check_username();
  //  password_validation::check_validity();
    

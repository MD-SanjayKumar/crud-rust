use std::io;
use std::collections::HashMap;
use crate::is_logged;
use crate::usr_details;

pub fn check_login(users:&mut HashMap<String,usr_details>){
    if unsafe { is_logged == true}{
        println!("Already logged in.");
      }else{
      let mut vx = String::new();
      let mut vy = String::new();
      println!("Enter Name:");
      io::stdin().read_line(&mut vx).expect("not found");
      println!("Enter Password:");
      io::stdin().read_line(&mut vy).expect("not found");
      match users.get(&vx) {
        Some(cf) => {
          if vy.trim() == cf.password.trim(){
            unsafe{is_logged = true;}
            println!("Logged in successfully.");
          }
          else {
              println!("Please check input.");
          }
        },
      None => println!("Please check input."),
    }
  }
}

pub fn check_logout(){
    if unsafe { is_logged == false}{
        println!("Already logged out.");
    }else{
        unsafe{is_logged = false;}
        println!("Logout successfully.");
    }
  }


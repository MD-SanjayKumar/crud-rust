use std::io;
use std::collections::HashMap;
use crate::is_logged;
use crate::usr_details;

pub fn remove_element(users:&mut HashMap<String,usr_details>){
    if unsafe { is_logged == false}{
        println!("Please login first.");
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
            users.remove(&vx);
            unsafe{is_logged = false;}
            println!("Deleted successfully.");
          }
          else {
              println!("Please check input.");
          }
        },
      None => println!("Please check input."),
    }
  }
}

pub fn update_element(users:&mut HashMap<String,usr_details>){
    if unsafe { is_logged == false}{
        println!("Please login first.");
      }else{
      let mut vx = String::new();
      let mut vy = String::new();
      let mut vy1 = String::new();
      println!("Enter Name:");
      io::stdin().read_line(&mut vx).expect("not found");
      println!("Enter Password:");
      io::stdin().read_line(&mut vy).expect("not found");
      println!("Enter New Address:");
      io::stdin().read_line(&mut vy1).expect("not found");
      match users.get(&vx) {
        Some(cf) => {
          if vy.trim() == cf.password.trim(){
            (users.get_mut(&vx).unwrap()).address = vy1;
            unsafe{is_logged = false;}
            println!("Updated successfully.");
          }
          else {
              println!("Please check input.");
          }
        },
      None => println!("Please check input."),
    }
  }
}
//use std::io;
use std::collections::HashMap;

pub fn newhash(){
    //let value1 = 10;
    let mut nhash = HashMap::new();
    // let mut val= String::new();
    // let mut val1 = String::new();
    // io::stdin().read_line(&mut val).expect("value not found");
    // io::stdin().read_line(&mut val1).expect("value not found");
     nhash.insert("Python", 1.0);
     nhash.insert("Swift", 8.0);
     nhash.insert("Java", 1.0);
     nhash.insert("C++", 3.0);
     nhash.insert("GoLang", 2.0);
    // for (k,v) in nhash.iter() {
    //     println!("{}, {}",k,v)

    println!("length is {}",nhash.len());
    for (k,v) in nhash.iter() {
        println!("Key: {}, Value: {}",k,v)
    }

    match nhash.get("C++") {
        Some(hs)=> println!("The value is {}",hs),
        None=> println!("Not found")
    }

    nhash.remove("C++");

    for (k, v) in &nhash{
        println!("Updated Key: {}, Value: {}",k,v);
    }

    println!("Does it include Swift? {}",nhash.contains_key("Swift"));

}
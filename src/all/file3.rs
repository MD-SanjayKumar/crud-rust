use std::io;

pub fn switch(){
    let st = io::stdin();
    let mut v = String::new();
    st.read_line(&mut v).expect("Value not found");
    match v.as_str().trim(){
        "A" => println!("Option 1 selected"),
        "B" => println!("Option 2 selected"),
        "C" => println!("Option 3 selected"),
        "D" => println!("Option 4 selected"),
        _ => println!("No option selected")
    }
}
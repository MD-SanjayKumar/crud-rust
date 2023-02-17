//mod file2;
use std::fs::File;
use std::io::prelude::*;


pub fn nfq() {
    //file2::newfun();
    let mut f1 = File::create("newfile.txt")
    .expect("File not created");
    f1.write_all(b"This is the new file which is created right now.")
    .expect("Failed to input data.");
}

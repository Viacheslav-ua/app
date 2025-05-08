use core::panic;
use std::fs::File;
use std::io::ErrorKind;


fn main() {
    let path: &str = "lol.html";
    let _path1: &str = "lol1.html";
    let _path2: &str = "lol2.html";

    let f = File::open(path);

    let _f  = match f {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(file) => file,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other => panic!("Problem opening the file: {:?}", other),  
        },
    };
    // _____________________________________________________

    let _f1 = File::open(_path1).unwrap();
    // Or
    let _f2 = File::open(_path2).expect( "Failed to open lol2.html");
}   
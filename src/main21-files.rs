use std::{fs::File, io::{Read}};


fn main() {
let path = "src/files/data.txt";

// let mut f =  File::create(path).expect("Failed to create file");

// f.write_all("Hello, data!".as_bytes()).expect("Error writing to file");
// f.write_all(b"Hello, data!")

let mut f = File::open(path).expect("Failed to open file");

let mut file_data = String::new();
f.read_to_string(&mut file_data).expect("Failed to read file");
println!("File data: {}", file_data);

}
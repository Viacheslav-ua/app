use std::{fs::OpenOptions, io::{Read, Write}};


fn main(){
  let path = "src/files/data.txt";
  
  let mut f = OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open(path)
    .expect("Error opening/creating file");

  let mut file_data = String::new();

  f.read_to_string(&mut file_data).expect("Error reading file");
  println!("File contents: {}", file_data);

  println!("Enter the text you want to append to the file: ");
  let mut input = String::new();
  std::io::stdin()
    .read_line(&mut input).expect("Error getting user input");

  f.write_all(input.as_bytes()).expect("Error writing to file");

  std::fs::rename(path, "src/files/text.txt1").expect("Error renaming file");

  std::fs::remove_file("src/files/text.txt").expect("Error deleting file");

}
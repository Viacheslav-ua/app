use std::{fs::File, io::{Error, ErrorKind, Read}};

fn main() {
  let path = "src/file.txt";

  match read_file_data(path) {
    Ok(data) => println!("File data: {}", data),
    Err(e) => match e.kind() {
      ErrorKind::NotFound => match File::create(path) {
        Ok(_) => println!("File created successfully."),
        Err(e) => panic!("Problem creating the file: {:?}", e),
      },
      other => panic!("ERROR OCCURRED: {:?}", other),
    },
  }
}

fn read_file_data(path: &str) -> Result<String, Error> {
  let mut f = File::open(path)?; // Or -

  // let mut f = match f {
  //   Ok(file) => file,
  //   Err(e) => return Err(e),
  // };

  let mut data = String::new();

  // match f.read_to_string(&mut data) {
  //   Ok(_) => Ok(data),
  //   Err(e) => Err(e),
  // }
  // Or -
  f.read_to_string(&mut data)?;
  Ok(data)
}

fn _read_file_data_correct(path: &str) -> Result<String, Error> {
  let mut data = String::new();
  File::open(path)?.read_to_string(&mut data)?;
  Ok(data)
}
use std::io;

fn main() {
  let mut name: String = String::new();

  match io::stdin().read_line(&mut name) {
      Ok(_n) => {
        println!("Hello {}: size - {}", name, _n)
      },
      Err(e) => {
        println!("ERROR - {}", e)
      }

  }
}
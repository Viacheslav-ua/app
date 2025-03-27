
use std::io;

fn main() {
  
      // ax^2 + bx + c = 0
      // D = b^2 - 4(a*c)

    let mut a_str: String = String::new();
    let mut b_str: String = String::new();
    let mut c_str: String = String::new();

    loop {
    println!("Enter argument A");
    match io::stdin().read_line(&mut a_str) {
      Ok(_) => {},
      Err(_e) => {
        println!("Input error-{}", _e)
      },
    }
    println!("Enter argument B");
    match io::stdin().read_line(&mut b_str) {
      Ok(_) => {},
      Err(_e) => {
        println!("Input error-{}", _e)
      },
    }
    println!("Enter argument C");
    match io::stdin().read_line(&mut c_str) {
      Ok(_) => {},
      Err(_e) => {
        println!("Input error-{}", _e)
      },
    }

    let a: f64 = a_str.trim().parse().unwrap();
    let b: f64 = b_str.trim().parse().unwrap();
    let c: f64 = c_str.trim().parse().unwrap();

    println!("{}, {}, {}", a, b, c);

    let d: f64 = (b*b) - 4.0*(a*c);

    if d > 0.0 {
      let x1: f64 = ((-b) + d.sqrt()) / (2.0*a);
      let x2: f64 = ((-b) - d.sqrt()) / (2.0*a);

      println!("Completed\n We have two roots\n D = {}\n root1 = {}\n root2 = {}", d, x1, x2);
    }

    if d == 0.0{
      let x: f64 = (-b)/(2.0*a);
      println!("Completed\n We have one root\n D = 0\n root = {}\n ", x);
    }

    if d < 0.0 {
      println!("Roots don't exist\n D<0\n D = {}", d);
    }
  }
}
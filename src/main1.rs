fn main() {
  let age: u8 = 30;
  let mut num: f64 = 3.23;
  let mut name: String = String::from("John");
  let text: String = String::from("Some text here");
  let symbol: char = 's';
  let logic: bool = true;
  let logic1: bool = false;

  name = String::from("John1");
  num = 4.87634;

  println!("My name is {}", name);
  println!("I'm {} years old", age);
  println!("This is float number {}", num);
  println!("{}", text);
  println!("{}", symbol);
  println!("{}, {}", logic, logic1)
}

fn main() {
  // let str1: String = String::from("Text");
  // let str3: String = String::from("Text3");
  // let str2: String = str1.clone();

  let s: String = give();

  // let s2: String = take_and_give(str1);

  // print_value(str1);
 
  print_value(&s);
  println!("long s is: {}", calc(&s));
  println!("s is: {}", s);
  println!("s2 is: {}", s);

  // println!("The value of str1 is: {}", str1);

  fn print_value(s: &String) {
    println!("The value of s is: {}", s);
  }

  fn give() -> String { String::from("given Value") }
  fn take_and_give(s: String) -> String { s }

  fn calc(s: &String) -> usize{ s.len() }
}
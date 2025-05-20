

fn main() {
  let n1 = 0xa; // 10 in decimal
  let n2 = 126345; 
  let n3 = 1;
  let list= vec![1, 2, 3, 4, 5];
  let tuple = (23, 56.87, "hello");

  println!("{2} {1} {0} {2}", n1, n2, n3);
  println!("{john} {rfnt} {gert} {gert}", john = n1, rfnt = n2, gert = n3);

  println!("Binary: {:b}\nOctal: 0o{:o}\nHexadecimal: 0x{:x}", n1, n2, n3);
  println!("Binary: {0:b}\nOctal: 0o{0:o}\nHexadecimal: 0x{0:x}", n1);

  println!("{:?}\n{:?}", list, tuple);
  println!("{1:#?}\n{0:#?}", list, tuple);

  eprintln!("It's an error message");
}

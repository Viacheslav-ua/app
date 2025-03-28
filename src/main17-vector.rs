

fn main() {
  let mut list: Vec<i32> =   vec![1, 2, 3, 4, 5];
  let mut list2: Vec<f64> = Vec::new();
  let mut listStr: Vec<String> = Vec::new();

  listStr.push("'value'".to_string());
  listStr.push("Good".to_string());

  let str: String = String::from("Hello");

  println!("List: {:?}", list);
  list.push(20);

  
  println!("List: {:?}", list);
  println!("List: {}", list[2]);
  list.remove(2);
  println!("List: {:?}", list);
  println!("List: {:?}", list.get(2));
  println!("List: {}", list.get(2).unwrap());

  println!("ListStr: {:?}", listStr.get(1).unwrap());
  println!("ListStr: {:?}", listStr[1]);
println!("{:?}", str);

  match list.get(4) {
      Some(x ) => println!("sdfswdef{}", x),
      None => println!("Not found"),
  }

  for el in &list {
      println!("{}", el);
  }

  for el in list.iter() {
      println!("{}", el);
  }

}
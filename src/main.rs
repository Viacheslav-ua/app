

fn main() {
  let mut list1 = vec![1, 2, 3, 4, 5];

  list1.push(6);
  list1.insert(2, 10);

  println!("List1: {:?}", list1);
  println!("List1-2: {:?}", &list1[2]);

  let index = 2;

  match list1.get(index) {
    Some(value) => println!("List1 el {1} is: {0:?}", value, index),
    None => println!("List1 element {}: None", index),
      
  }
  println!("List1 element {1} is: {0:?}", list1.get(index).unwrap(), index);

  println!("{}", list1.first().unwrap());

  let list2: Vec<i32> = vec![];
  match list2.first() {
    Some(el) => println!("First el is {}", el),
    None => println!("Vector is empty"),
  }

  println!("Last el is {}", &list1[list1.len() - 1]);
  match  list2.last() {
    Some(el) => println!("Last el is {}", el),
    None => println!("Vector is empty"),
  }

  if list2.is_empty() {
    println!("Vector is empty")
  } else {
    println!("Vector is not empty")
  }

}
use std::collections::HashMap;

fn main() {
    let mut map: HashMap<&String, i32> = HashMap::new();

    let n1: String = "Denis".to_string();
    let n2: String = "Ali".to_string();
    let n3: String = "John".to_string();
    let n4: String = "Vlad".to_string();
    map.insert(&n1, 10);
    map.insert(&n2, 20);
    map.insert(&n3, 9);

    println!("{:?}", map);
    println!("{}", map[&n1]);
    
    match map.get(&n2) {
        Some(value) => println!("Value: {}", value),
        None => println!("Key not found"),
        
    }

    for (name, value) in &map {
        println!("Name: {}, Value: {}", name, value);
    }
    
    map.entry(&n4).or_insert(8);
    map.entry(&n3).or_insert(15);
    println!("{:?}", map);

    let s = "Learn Rust with me learn".to_lowercase();
    let mut count_map: HashMap<&str, i32> = HashMap::new();

    for w in s.split_whitespace() {

       let count: &mut i32 = count_map.entry(w).or_insert(0);
        *count += 1; 
    }

    println!("{:?}", count_map);

}
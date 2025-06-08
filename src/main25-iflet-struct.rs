struct User {
    name: String,
    age: i8,
}

fn main() {
    let n = "John".to_string();
    let user = User {
        name: n,
        age: 25,
    };

    if let User { name: n, age: x } = user {
        println!("Matched {}", x)
    }
}
fn main() {

    let a = 1..5;
    let b = 1..=5;

    for i in a {
        print!("{}", i)
    }
    println!();
    println!("--------");

    for i in b {
        print!("{}", i)
    }
    println!();

    println!();
    println!("--------");

    for i in (1..=8).rev() {
        print!("{}", i)
    }
    println!();

    let numbers: Vec<i32> = (1..=10).rev().collect();
    for i in numbers {
        println!("{}", i)
    }

    let slice = &numbers[2..7];

}
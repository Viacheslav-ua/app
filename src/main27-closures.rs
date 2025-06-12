fn main() {
    let is_even = |x| {
        println!("filtering {}", x);
        x % 2 == 0
    };

    println!("Is 4 even? {}", is_even(4));
    println!("Is 5 even? {}", is_even(5));

    let mut y = 10;

    let add = move |x| {
        println!("adding {} and {}", x, y);
        x + y
    };

    let z = &mut y;

    println!("1 + 2 = {}", add(2));
}
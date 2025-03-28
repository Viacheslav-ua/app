#[derive(Debug)]
enum Person {
    Adult,
    Underage,
}

enum Sey {
    Hi(String),
    Bye(String),
    GM(String),
    GN(String),
}

enum MathOperation {
    Add(f64, f64),
    Sub(f64, f64),
    Mul(f64, f64),
    Div(f64, f64),
}


fn main() {
    let person: Person = Person::Adult;

    match person {
        Person::Adult => println!("Come on"),
        Person::Underage => println!("You are forbidden"),
    }

    let sey: Sey = Sey::Hi("Hello".to_string());

    match sey {
        Sey::Hi(h) => println!("{}", h),
        Sey::Bye(msg) => println!("{}", msg),
        _=> println!("Default"),
    }

    let mo: MathOperation = MathOperation::Add(2.0, 3.0);
    match mo {
        MathOperation::Add(a, b) => println!("{}", a + b),
        MathOperation::Sub(a, b) => println!("{}", a - b),
        MathOperation::Mul(a, b) => println!("{}", a * b),
        MathOperation::Div(a, b) => println!("{}", a / b),
    }   
}
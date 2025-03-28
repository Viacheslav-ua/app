#[derive(Debug)]

// struct Person {
//     name: String,
//     surname: String,
//     age: u8,
//     balance: f64,
// }

struct Triangle {
    cat1: f64,
    cat2: f64,
}

impl Triangle {
    fn hypotenuse(&self) -> f64 {
        (self.cat1.powf(2.0) + self.cat2.powf(2.0)).sqrt()
    }

    fn area(&self) -> f64 {
        (self.cat1 * self.cat2) / 2.0
    }

    fn create_asc(cat: f64) -> Triangle {
        Triangle { cat1: cat, cat2: cat }
    }
}
fn main() {

    let t = Triangle { cat1: 3.0, cat2: 4.0 };
    println!("hypotenuse is: {}, area is: {}", t.hypotenuse(), t.area());

    let asc_t: Triangle = Triangle::create_asc(10.0);

    println!("hypotenuse is: {}, area is: {}", asc_t.hypotenuse(), asc_t.area());
    
    

    // // let mut person1 = Person {
    // //     name: String::from("John"),
    // //     surname: "Doe".to_string(),
    // //     age: 30,
    // //     balance: 3100.67,
    // // };

    // // person1.name = "Jeff".to_string();
    // // println!("{}:{:#?}", person1.name, person1);

    // let age = 30;
    // let balance = 3100.67;
    // let person1 = Person {
    //     name: String::from("John"),
    //     surname: String::from("Doe"),
    //     age,
    //     balance,
    // };
    // let person2 = Person {
    //     name: String::from("Jeff"),
    //     surname: String::from("Smith"),
    //     ..person1
    // };
    // println!("{}:{:#?}", person1.name, person1);
    // println!("{}:{:#?}", person2.name, person2);
}

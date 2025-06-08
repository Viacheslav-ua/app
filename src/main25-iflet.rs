    enum Time {
        Morning(String),
        Afternoon(String),
        Evening(String),
    }

fn main() {
    let nums = (1, 2, 3, 4, 5);

    if let (1, 2, b, c, d) = nums {
       println!("Yes! {} {} {}", b, c, d) 
    } else {
        println!("No!")
    }


    let t = Time::Morning("Hello user".to_string());

    match &t {
        Time::Morning(m) => println!("M {}", m),
        Time::Afternoon(m) => println!("A {}", m),
        Time::Evening(m) => println!("E {}", m),
    }

    if let Time::Morning(m) = &t {
        println!("M1 {}", m)
    } else {
        println!("Not morning")
    }

    let num: Option<i32> = Some(7);

    match num {
        Some(n) => println!("Some {}", n),
        None => println!("None"),
    }

    if let Some(n) = num  {
        println!("Some if let {}", n);
    }
    if let Some(7) = num  {
        println!("Nice");
    }

    let num_1 = "7";
    let mut num_int = 0;
    let parsed = num_1.parse::<i32>();

    // match parsed {
    //     Ok(n) => {
    //         num_int = n
    //     },
    //     Err(_) => println!("Enter the number"),
    // }

    if let Ok(n) = parsed {
        num_int = n
    }

    println!("{}", num_int)


}
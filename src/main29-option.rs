
fn main() {
    let option = Some(7);
    let option1: Option<i32> = None;

    println!("{}", option.is_some());
    println!("{}", option.is_none());

    println!("{}", option.unwrap());
    // println!("{}", option1.unwrap());
    // println!("{}", option1.expect("Option is undefined"));

    println!("{}", option1.unwrap_or_default());
    println!("{}", option1.unwrap_or(777));

    println!("{}", option.unwrap_or_default());

    println!("{}", option1.unwrap_or_else(|| {
        println!("unwrap_or_else");
        0
    }));

    println!("{:?}", option1.ok_or("err - option is none"));
    println!("{:?}", option.ok_or("err - option is none"));

    let op1: Option<i32> = Some(10);
    let op2: Option<i32> = Some(15);

    println!("{:?}", op1.and(op2));

    println!("{:?}", option.map(|num| num * num).unwrap());
    println!("{:?}", option1.map(|num| num * num).unwrap_or(-1));

    println!("{:?}", op1.filter(|num| num & 1 == 0));
    println!("{:?}", op2.filter(|num| num & 1 == 0));

    println!("{:?}", op1.and_then(|n| Some((n as f32).sqrt())));

}

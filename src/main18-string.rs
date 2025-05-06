fn main(){
    let str1: String = String::new();
    let mut str2: String = String::from("Hello");
    let str3: String = "World".to_string();
    let str31: String = "world".to_string();

    let s: String = "TEXT".to_string();
    let s1: String = "ТЕКСТ".to_string();

    let str4: String = String::from("programs");

    str2.push_str(" World");
    str2.push('!');

    println!("{}", str2);

    let result1: String = str3 + " of " + &str4; // str3 x => result
    let result2: String = format!("{} of {}", str31, str4);


    println!("{}", result1);
    println!("{}", result2);

    for ch in s.chars() {
        println!("{}", ch);
    }

    for b in s.bytes() {
        println!("{}", b);
    } 

    for b in s1.bytes() {
        println!("{}", b);
    }
}

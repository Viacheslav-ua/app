fn main() {
    let data = vec![
        vec!["Den", "11", "12", "10"],
        vec!["Kate", "9", "12", "13"],
        vec!["John", "10", "11", "12"],
        vec!["Alice", "8", "4", "9"],
    ];

    for mut student in data {
        println!("Scores for {}:", student[0]);
        while let Some(value) = student.pop() {
            if let Ok(result) = value.parse::<i32>() {
                println!("{}", result);
            } else {
                println!("End");
            
            }
        }
    }
}

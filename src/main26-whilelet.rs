fn  main() {
    let mut counter = Some(0);

    loop {
        match &mut counter {
            Some(value) => {
                if *value == 31 {
                    println!("End");
                    counter = None; // Set to None to exit the loop
                } else {
                    println!("Current counter value: {}", value);
                    counter = Some(*value + 1);
                }
            },
            None => {
                println!("Counter is None, exiting loop.");
                counter = Some(0);
                break;
            }
        }
    
    }

    while let Some(value) = counter {
        if value == 31 {
            println!("End_2");
            counter = None; // Set to None to exit the loop
        } else {
            println!("Current counter value: {}", value);
            counter = Some(value + 1);
        }
        
    }
}
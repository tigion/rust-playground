use std::io::{self, Write};

pub fn read_str(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_owned()
}

pub fn read_i32(prompt: &str) -> i32 {
    let mut input = String::new();

    loop {
        print!("{}", prompt);
        io::stdout().flush().expect("Failed to flush stdout");

        input.clear();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<i32>() {
            Ok(value) => break value,
            Err(_) => {
                println!("Please enter a valid integer. Try again.");
            }
        }
    }
}

pub fn read_f64(prompt: &str) -> f64 {
    let mut input = String::new();

    loop {
        print!("{}", prompt);
        io::stdout().flush().expect("Failed to flush stdout");

        input.clear();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<f64>() {
            Ok(value) => break value,
            Err(_) => {
                println!("Please enter a valid number. Try again.");
            }
        }
    }
}

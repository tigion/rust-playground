use std::io::{self, Write};

fn read_product_name(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_owned()
}

fn read_product_amount(prompt: &str) -> i32 {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input
        .trim()
        .parse()
        .expect("Failed to parse input to integer")
}

fn read_product_price(prompt: &str) -> f64 {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input
        .trim()
        .parse()
        .expect("Failed to parse input to float")
}

fn main() {
    // Read product name:
    let name = read_product_name("Enter product name: ");

    // Read product amount:
    let amount = read_product_amount("Enter product amount: ");

    // Read product price:
    let price = read_product_price("Enter product price: ");

    // Display product information:
    println!("\nProduct name: {name}");
    println!("Product amount: {amount}");
    println!("Product price: {price:.2}");
}

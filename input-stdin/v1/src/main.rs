use std::io;

fn main() {
    // Read product name:
    println!("Enter product name: ");

    let mut input_name = String::new();
    io::stdin()
        .read_line(&mut input_name)
        .expect("Failed to read input");

    let name = input_name.trim().to_owned();

    // Read product amount:
    println!("Enter product amount: ");

    let mut input_amount = String::new();
    io::stdin()
        .read_line(&mut input_amount)
        .expect("Failed to read input");

    let amount: i32 = input_amount
        .trim()
        .parse()
        .expect("Failed to parse input to integer");

    // Read product price:
    println!("Enter product price: ");

    let mut input_price = String::new();
    io::stdin()
        .read_line(&mut input_price)
        .expect("Failed to read input");

    let price: f64 = input_price
        .trim()
        .parse()
        .expect("Failed to parse input to float");

    // Display product information:
    println!();
    println!("Product name: {}", name);
    println!("Product amount: {}", amount);
    println!("Product price: {:.2}", price);
}

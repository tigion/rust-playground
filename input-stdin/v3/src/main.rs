mod input;

use input::{read_f64, read_i32, read_str};

fn read_product_name() -> String {
    loop {
        let name = read_str("Enter product name: ");

        if !name.trim().is_empty() {
            return name;
        }

        println!("Product name cannot be empty. Please try again.");
    }
}

fn read_product_amount() -> i32 {
    loop {
        let amount = read_i32("Enter product amount: ");

        if amount >= 0 {
            return amount;
        }

        println!("Product amount cannot be negative. Please try again.");
    }
}

fn read_product_price() -> f64 {
    loop {
        let price = read_f64("Enter product price: ");

        if price >= 0.0 {
            return price;
        }

        println!("Product price cannot be negative. Please try again.");
    }
}

fn main() {
    // Read product name:
    let name = read_product_name();

    // Read product amount:
    let amount = read_product_amount();

    // Read product price:
    let price = read_product_price();

    // Display product information:
    println!("\nProduct name: {name}");
    println!("Product amount: {amount}");
    println!("Product price: {price:.2}");
}

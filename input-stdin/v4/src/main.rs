use std::io;

mod input;

use input::{read_f64, read_i32, read_str};

fn read_product_name() -> io::Result<String> {
    loop {
        let name = read_str("Enter product name: ")?;

        if !name.trim().is_empty() {
            return Ok(name);
        }

        println!("Product name cannot be empty. Please try again.");
    }
}

fn read_product_amount() -> io::Result<i32> {
    loop {
        let amount = read_i32("Enter product amount: ")?;

        if amount >= 0 {
            return Ok(amount);
        }

        println!("Product amount cannot be negative. Please try again.");
    }
}

fn read_product_price() -> io::Result<f64> {
    loop {
        let price = read_f64("Enter product price: ")?;

        if price >= 0.0 {
            return Ok(price);
        }

        println!("Product price cannot be negative. Please try again.");
    }
}

fn display_product_info(name: &str, amount: i32, price: f64) {
    println!("\nProduct name: {name}");
    println!("Product amount: {amount}");
    println!("Product price: {price:.2}");
}

fn run() -> io::Result<()> {
    let name = read_product_name()?;
    let amount = read_product_amount()?;
    let price = read_product_price()?;

    display_product_info(&name, amount, price);

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
    }
}

use std::io;

use crate::input::read;

pub fn read_product_name() -> io::Result<String> {
    loop {
        let name: String = read("Enter product name: ")?;

        // if !name.is_empty() {
        if !name.trim().is_empty() {
            return Ok(name);
        }

        println!("Product name cannot be empty. Please try again.");
    }
}

pub fn read_product_amount() -> io::Result<i32> {
    loop {
        let amount: i32 = read("Enter product amount: ")?;

        if amount >= 0 {
            return Ok(amount);
        }

        println!("Product amount cannot be negative. Please try again.");
    }
}

pub fn read_product_price() -> io::Result<f64> {
    loop {
        let price: f64 = read("Enter product price: ")?;

        if price >= 0.0 {
            return Ok(price);
        }

        println!("Product price cannot be negative. Please try again.");
    }
}

pub fn display_product_info(name: &str, amount: i32, price: f64) {
    println!("\nProduct name: {name}");
    println!("Product amount: {amount}");
    println!("Product price: {price:.2}");
}

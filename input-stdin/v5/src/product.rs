use std::io;

use crate::input::{read_f64, read_i32, read_str};

pub fn read_product_name() -> io::Result<String> {
    loop {
        let name = read_str("Enter product name: ")?;

        if !name.trim().is_empty() {
            return Ok(name);
        }

        println!("Product name cannot be empty. Please try again.");
    }
}

pub fn read_product_amount() -> io::Result<i32> {
    loop {
        let amount = read_i32("Enter product amount: ")?;

        if amount >= 0 {
            return Ok(amount);
        }

        println!("Product amount cannot be negative. Please try again.");
    }
}

pub fn read_product_price() -> io::Result<f64> {
    loop {
        let price = read_f64("Enter product price: ")?;

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

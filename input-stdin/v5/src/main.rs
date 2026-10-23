use std::io;

mod input;
mod product;

use product::{display_product_info, read_product_amount, read_product_name, read_product_price};

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

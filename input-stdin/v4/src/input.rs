use std::io::{self, Write};

pub fn read_str(prompt: &str) -> io::Result<String> {
    print!("{prompt}");
    io::stdout().flush()?;

    let mut input = String::new();
    let bytes_read = io::stdin().read_line(&mut input)?;

    if bytes_read == 0 {
        return Err(io::Error::from(io::ErrorKind::UnexpectedEof));
    }

    Ok(input.trim().to_string())
}

pub fn read_i32(prompt: &str) -> io::Result<i32> {
    let mut input = String::new();

    loop {
        print!("{prompt}");
        io::stdout().flush()?;

        input.clear();
        let bytes_read = io::stdin().read_line(&mut input)?;

        if bytes_read == 0 {
            return Err(io::Error::from(io::ErrorKind::UnexpectedEof));
        }

        if let Ok(value) = input.trim().parse::<i32>() {
            break Ok(value);
        }

        println!("Please enter a valid integer number. Try again.");
    }
}

pub fn read_f64(prompt: &str) -> io::Result<f64> {
    let mut input = String::new();

    loop {
        print!("{prompt}");
        io::stdout().flush()?;

        input.clear();
        let bytes_read = io::stdin().read_line(&mut input)?;

        if bytes_read == 0 {
            return Err(io::Error::from(io::ErrorKind::UnexpectedEof));
        }

        if let Ok(value) = input.trim().parse::<f64>()
            && value.is_finite()
        {
            break Ok(value);
        }

        println!("Please enter a valid number. Try again.");
    }
}

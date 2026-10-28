use std::{
    io::{self, Write},
    str::FromStr,
};

pub fn read<T: FromStr>(prompt: &str) -> io::Result<T> {
    loop {
        print!("{prompt}");
        io::stdout().flush()?;

        let mut input = String::new();
        if io::stdin().read_line(&mut input)? == 0 {
            // break Err(io::Error::from(io::ErrorKind::UnexpectedEof));
            // break Err(io::ErrorKind::UnexpectedEof.into());
            return Err(io::ErrorKind::UnexpectedEof.into());
        }

        match input.trim().parse() {
            // Ok(value) => break Ok(value),
            Ok(value) => return Ok(value),
            Err(_) => println!("Invalid input, please enter again."),
        }
    }
}

use std::{fs::File, error::Error};

fn main() -> Result<(), Box<dyn Error>>{
    let f = File::open("hello.txt")?;

    Ok(())
}
use std::{fs, io::Read, error::Error, process};

fn read_file() -> Result<String, Box<dyn Error>> {
    let mut f = fs::File::open("sample.s")?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf)
}

fn word_position(contents: String) {
    let mut text = String::new();
    let mut p = 0;
    for word in contents.lines() {
        println!("{p:02X}: {word}");
        text.push_str(word);
        p+=word.len();
    }
    println!("{:02X}", text.len());
}

fn main() {
    let contents = read_file().unwrap_or_else(
        |err| {
            eprintln!("Error!\n{err}");
            process::exit(1);
        }
    );
    word_position(contents);
}
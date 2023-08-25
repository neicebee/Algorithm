use std::{fs, io::Read, error::Error, process};

fn read_file() -> Result<String, Box<dyn Error>> {
    let mut f = fs::File::open("sample.txt")?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(contents)
}

fn main() {
    let contents = read_file().unwrap_or_else(
        |err| {
            eprintln!("Error!\n{}", err);
            process::exit(1);
        }
    );
    let split_contents = contents.split_ascii_whitespace().collect::<Vec<&str>>();
    let (mut n, mut a, mut etc) = (0, 0, 0);

    for word in split_contents {
        if let Err(_) = word.parse::<i32>() {
            a+=1;
            for x in word.chars() {
                if !x.is_alphabetic() {
                    etc+=1;
                    a-=1;
                    break;
                }
            }
        } else {
            n+=1;
        }
    }

    println!(
        "Number = {}\nAlphabet = {}\nOther = {}",
        n, a, etc
    );
}
use std::{fs, io::Read, error::Error, process};

struct Optab<'a> {
    name: &'a str,
    len: i32,
}

impl<'a> Optab<'a> {
    fn new(name: &'a str, len: i32) -> Optab<'a> {
        Optab { name, len }
    }
}

fn read_file() -> Result<String, Box<dyn Error>> {
    let mut f = fs::File::open("command.s")?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf)
}

fn word_checking(contents: String) {
    let mut location = 0;
    let mut l = 1;
    let wordtab: Vec<Optab> = vec![
        Optab::new("LDA", 3), Optab::new("STA", 4), Optab::new("ADD", 5),
        Optab::new("TIX", 2), Optab::new("CMP", 6)
    ];
    for word in contents.lines() {
        let mut checker = true;
        for x in wordtab.iter() {
            if x.name == word {
                println!(
                    "{l:>2}, {location:02X}, {}, {:#02}",
                    x.name, x.len
                );
                location+=x.len;
                l+=1;
                checker = false;
                break;
            }
        }
        if checker == true {
            println!(" Undefined word");
        }
    }
}

fn main() {
    let contents = read_file().unwrap_or_else(
        |err| {
            eprintln!("Error!\n{err}");
            process::exit(1);
        }
    );
    word_checking(contents);
}
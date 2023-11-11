use std::{io::Read, fs, error::Error, process};

fn read_file(file_name: &str) -> Result<String, Box<dyn Error>>{
    let mut f = fs::File::open(file_name)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf)
}

fn has_more_commands(contents: String) {
    for l in contents.lines() {
        if let Some(i) = l.find("//") {
            command_type(&l[..i].trim());
        } else {
            command_type(l.trim());
        }
    }
}

fn command_type(command: &str) {

}

fn main() {
    let contents = read_file("prog.asm").unwrap_or_else(
        |err| {
            eprintln!("Error!\n{err}");
            process::exit(1);
        }
    );
    has_more_commands(contents);
}
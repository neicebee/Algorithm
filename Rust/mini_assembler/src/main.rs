use std::{io::Read, fs, error::Error, process};
use std::fmt::Debug;

const A: char =  '@';
const LABEL: [char; 2] = ['(', ')'];

#[derive(Debug)]
#[derive(PartialEq)]
enum Command {
    A,
    C,
    Label,
}

#[derive(Debug)]
struct Cmds {
    kind: Command,
    content: String,
}

#[derive(Debug)]
struct CCommand<'a> {
    dest: &'a str,
    comp: &'a str,
    jump: &'a str,
}

fn read_file(file_name: &str) -> Result<String, Box<dyn Error>>{
    let mut f = fs::File::open(file_name)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf)
}

fn command_type(cmd: &str) -> Cmds {
    if cmd.contains(A) {
        Cmds {
            kind: Command::A,
            content: cmd.replace(A, ""),
        }
    } else if cmd.contains(LABEL) {
        Cmds {
            kind: Command::Label,
            content: cmd.replace(LABEL, ""),
        }
    } else {
        Cmds {
            kind: Command::C,
            content: cmd.to_string(),
        }
    }
}

fn c_field<'a>(i: &'a String) -> Option<CCommand<'a>> {
    let (mut d, mut j) = (0, 0);
    if i.contains('=') { d+=1; }
    if i.contains(';') { j+=1; }
    match (d, j) {
        (1, 0) => {
            let mut it = i.split('=');
            Some(CCommand {
                dest: it.next().unwrap(),
                comp: it.next().unwrap(),
                jump: "",
            })
        },
        (0, 1) => {
            let mut it = i.split(';');
            Some(CCommand {
                dest: "",
                comp: it.next().unwrap(),
                jump: it.next().unwrap(),
            })
        },
        (1, 1) => {
            let mut it = i.split(['=', ';']);
            Some(CCommand {
                dest: it.next().unwrap(),
                comp: it.next().unwrap(),
                jump: it.next().unwrap(),
            })
        },
        _ => None,
    }
}

fn code(c: &CCommand) {
    let d = match c.dest {
        "null0" => "000",
        "M" => "001",
        "D" => "010",
        "MD" => "011",
        "A" => "100",
        "AM" => "101",
        "AD" => "110",
        "AMD" => "111",
        _ => "",
    };
    let j = match c.jump {
        "null" => "000",
        "JGT" => "001",
        "JEQ" => "010",
        "JGE" => "011",
        "JLT" => "100",
        "JNE" => "101",
        "JLE" => "110",
        "JMP" => "111",
        _ => "",
    };
    let c = match c.comp {
        "0" => "0101010",
        "1" => "0111111",
        "-1" => "0111010",
        "D" => "0001100",
        "A" => "0110000",
        "!D" => "0001101",
        "!A" => "0110001",
        "-D" => "0001111",
        "-A" => "0110011",
        "D+1" => "0011111",
        "A+1" => "0110111",
        "D-1" => "0001110",
        "A-1" => "0110010",
        "D+A" => "0000010",
        "D-A" => "0010011",
        "A-D" => "0000111",
        "D&A" => "0000000",
        "D|A" => "0010101",
        "M" => "1110000",
        "!M" => "1110001",
        "-M" => "1110011",
        "M+1" => "1110111",
        "M-1" => "1110010",
        "D+M" => "1000010",
        "D-M" => "1010011",
        "M-D" => "1000111",
        "D&M" => "1000000",
        "D|M" => "1010101",
        _ => "",
    };
    let binary_code = format!("111{c}{d}{j}");
    println!("{binary_code}");
}

fn main() {
    let contents = read_file("prog.asm").unwrap_or_else(
        |err| {
            eprintln!("Error!\n{err}");
            process::exit(1);
        }
    );

    for l in contents.lines() {
        if l.find("//")==Some(0) { continue }
        let c;
        if let Some(i) = l.find("//") {
            c = command_type(&l[..i].trim());
            println!("{:?}", c);
        } else {
            c = command_type(l.trim());
            println!("{:?}", c);
        }

        if c.kind == Command::C {
            if let Some(r) = c_field(&c.content) {
                println!("{:?}", r);
                code(&r);
            }
        }
    }
}
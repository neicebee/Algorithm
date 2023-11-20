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

struct C_Command<'a> {
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

fn c_field<'a>(i: &'a String) -> Option<C_Command<'a>> {
    let (mut d, mut j) = (0, 0);
    if i.contains('=') { d+=1; }
    if i.contains(';') { j+=1; }
    match (d, j) {
        (1, 0) => {
            let mut it = i.split('=');
            Some(C_Command {
                dest: it.next().unwrap(),
                comp: it.next().unwrap(),
                jump: "",
            })
        },
        (0, 1) => {
            let mut it = i.split(';');
            Some(C_Command {
                dest: "",
                comp: it.next().unwrap(),
                jump: it.next().unwrap(),
            })
        },
        (1, 1) => {
            let mut it = i.split(['=', ';']);
            Some(C_Command {
                dest: it.next().unwrap(),
                comp: it.next().unwrap(),
                jump: it.next().unwrap(),
            })
        },
        _ => None,
    }
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
        let mut c;
        if let Some(i) = l.find("//") {
            c = command_type(&l[..i].trim());
            println!("{:?}", c);
        } else {
            c = command_type(l.trim());
            println!("{:?}", c);
        }

        if c.kind == Command::C {
            let result = c_field(&c.content);
        }
    }
}
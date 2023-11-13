use std::{io::Read, fs, error::Error, process};

const A: char =  '@';
const LABEL: [char; 2] = ['(', ')'];

#[derive(Debug)]
struct Cmds {
    a_cmd: Vec<String>,
    c_cmd: Vec<String>,
    label: Vec<String>,
}

fn read_file(file_name: &str) -> Result<String, Box<dyn Error>>{
    let mut f = fs::File::open(file_name)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf)
}

fn has_more_commands(contents: String, c: &mut Cmds) {
    for l in contents.lines() {
        if let Some(i) = l.find("//") {
            if &l[..i].trim().len()!=&0 {
                command_type(&l[..i].trim(), c);
            }
        } else {
            command_type(l.trim(), c);
        }
    }
}

fn command_type(cmd: &str, c: &mut Cmds) {
    if cmd.contains(A) {
        c.a_cmd.push(
            cmd.replace(A, "")
        );
    } else if cmd.contains(LABEL) {
        c.label.push(
            cmd.replace(LABEL, "")
        );
    } else {
        c.c_cmd.push(
            cmd.to_string()
        );
    }
}

fn comp() {

}

fn dest() {

}

fn jump() {

}

fn main() {
    let contents = read_file("prog.asm").unwrap_or_else(
        |err| {
            eprintln!("Error!\n{err}");
            process::exit(1);
        }
    );
    let mut c = Cmds{
        a_cmd: vec![],
        c_cmd: vec![],
        label: vec![],
    };
    has_more_commands(contents, &mut c);
    
    for i in c.c_cmd {
        let (mut d, mut j) = (0, 0);
        if i.contains('=') { d+=1; }
        if i.contains(';') { j+=1; }
        match (d, j) {
            (1, 0) => {
                let t = i.split('=');
                println!("dust: {}\ncomp: {}",
                    t.next().unwrap(), t.next().unwrap()
                );
            },
            (0, 1) => {
                let t = i.split(';');
                
            },
            (1, 1) => ,
            _ => ,
        }
    }
}
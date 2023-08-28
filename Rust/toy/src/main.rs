/*
숫자 단어들에 대한 unsigned 정수를 구한 후 전체 합 출력
X'...' 형태 : 16진수
C'...' 형태 : ASCII 코드로 이루어진 16진수
*/

use std::{fs, io::Read, error::Error, process};

fn read_file() -> Result<String, Box<dyn Error>> {
    let mut f = fs::File::open("sample.s")?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(contents)
}

fn split_contents<'a>(contents: &'a str) -> (Vec<&'a str>, Vec<&'a str>, Vec<&'a str>) {
    let mut symbol = vec![];
    let mut command = vec![];
    let mut operand = vec![];
    for line in contents.lines() {
        let words = line.split("\t").collect::<Vec<&str>>();
        if words.len() == 4 {
            symbol.push(words[1]);
            command.push(words[2]);
            operand.push(words[3]);
        }
    }
    (symbol, command, operand)
}

fn print(args: Vec<&str>) {
    for i in 0..args.len() {
        if i%10==9 {
            println!("{:10}", args[i]);
        } else {
            print!("{:10} ", args[i]);
        }
    }
}

fn main() {
    let contents = read_file().unwrap_or_else(
        |err| {
            eprintln!("Error!\n{}", err);
            process::exit(1);
        }
    );
    let (symbol, command, operand) = split_contents(&contents);
    println!("Symbol:");
    print(symbol);
    println!("\nCommand:");
    print(command);
    println!("\nOperand:");
    print(operand);
}
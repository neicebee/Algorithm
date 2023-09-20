/*
숫자 단어들에 대한 unsigned 정수를 구한 후 전체 합 출력
X'...' 형태 : 16진수
C'...' 형태 : ASCII 코드로 이루어진 16진수
*/

use std::{fs, io::Read, error::Error, process};
use regex::Regex;

fn read_file() -> Result<String, Box<dyn Error>> {
    let mut f = fs::File::open("numb.s")?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    Ok(contents)
}

fn split_contents(contents: &str) -> (Vec<u64>, Vec<u64>, Vec<u64>) {
    let mut hex = vec![];
    let mut ascii_hex = vec![];
    let mut dec = vec![];
    let re1 = Regex::new(r"X'([A-Z0-9]+?)'").unwrap();
    let re2 = Regex::new(r"C'([A-Z0-9]+?)'").unwrap();
    for token in contents.split_whitespace() {
        if re1.is_match(token) {
            let tmp = re1.captures(token).unwrap().get(1).unwrap().as_str();
            hex.push(u64::from_str_radix(tmp, 16).unwrap());
            println!("token: {token}\t\t\tdec: {}", u64::from_str_radix(tmp, 16).unwrap());
        } else if re2.is_match(token) {
            let mut t = String::new();
            let tmp = re2.captures(token).unwrap().get(1).unwrap().as_str();
            for b in tmp.bytes() {
                t.push_str(&format!("{b:X}"));
            }
            ascii_hex.push(u64::from_str_radix(&t, 16).unwrap());
            println!("token: {token}\thex: {t}\tdec: {}", u64::from_str_radix(&t, 16).unwrap());
        } else {
            println!("token: {token}\t\t\tdec: {token}");
            dec.push(token.parse::<u64>().unwrap());
        }
    }
    (hex, ascii_hex, dec)
}

fn main() {
    let contents = read_file().unwrap_or_else(
        |err| {
            eprintln!("Error!\n{err}");
            process::exit(1);
        }
    );
    let (hex, ascii_hex, dec) = split_contents(&contents);
    println!(
        "\nsum = {}",
        hex.iter().sum::<u64>() + ascii_hex.iter().sum::<u64>() + dec.iter().sum::<u64>()
    )
}

// use std::{fs, io::Read, error::Error, process};

// fn read_file() -> Result<String, Box<dyn Error>> {
//     let mut f = fs::File::open("sample.s")?;
//     let mut contents = String::new();
//     f.read_to_string(&mut contents)?;

//     Ok(contents)
// }

// fn split_contents<'a>(contents: &'a str) -> (Vec<&'a str>, Vec<&'a str>, Vec<&'a str>) {
//     let mut symbol = vec![];
//     let mut command = vec![];
//     let mut operand = vec![];
//     for line in contents.lines() {
//         let words = line.split("\t").collect::<Vec<&str>>();
//         if words.len() == 4 {
//             symbol.push(words[1]);
//             command.push(words[2]);
//             operand.push(words[3]);
//         }
//     }
//     (symbol, command, operand)
// }

// fn print(args: Vec<&str>) {
//     for i in 0..args.len() {
//         if i%10==9 {
//             println!("{:10}", args[i]);
//         } else {
//             print!("{:10} ", args[i]);
//         }
//     }
// }

// fn main() {
//     let contents = read_file().unwrap_or_else(
//         |err| {
//             eprintln!("Error!\n{}", err);
//             process::exit(1);
//         }
//     );
//     let (symbol, command, operand) = split_contents(&contents);
//     println!("Symbol:");
//     print(symbol);
//     println!("\nCommand:");
//     print(command);
//     println!("\nOperand:");
//     print(operand);
// }
// use std::{io::Read, fs, error::Error, process};

// fn read_file() -> Result<String, Box<dyn Error>>{
//     let mut f = fs::File::open("src/data.txt")?;
//     let mut buf = String::new();
//     f.read_to_string(&mut buf)?;
//     Ok(buf)
// }

// fn main() {
//     let contents = read_file().unwrap_or_else(
//         |err| {
//             eprintln!("Error!\n{err}");
//             process::exit(1);
//         }
//     );

//     for l in contents.lines() {
//         if l=="-1" { break; }
//         let a = l.trim().parse::<u32>().unwrap();
//         let mut factors: Vec<u32> = (1..=a).filter(|x| a%x==0).collect();
//         factors.pop();
//         if a == factors.iter().sum() {
//             println!("{a} = {}", 
//                 factors.iter().map(
//                     |x| x.to_string()
//                 ).collect::<Vec<String>>().join(" + ")
//             );
//         } else {
//             println!("{a} is NOT perfect.");
//         }
//     }
// }

use std::{io, io::Read, error::Error};

fn main() -> Result<(), Box<dyn Error>>{
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    for l in buf.lines() {
        if l=="-1" { break; }
        let a = l.trim().parse::<u32>().unwrap();
        let mut f: Vec<u32> = (1..=a).filter(|x| a%x==0).collect();
        f.pop();
        if a==f.iter().sum() {
            println!("{a} = {}",
                f.iter().map(
                    |x| x.to_string()
                ).collect::<Vec<String>>().join(" + ")
            );
        } else {
            println!("{a} is NOT perfect.");
        }
    }
    Ok(())
}
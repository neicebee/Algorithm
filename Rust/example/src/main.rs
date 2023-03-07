use std::{fs::File, io::ErrorKind};

fn main() {
    let f = match File::open("hello.txt") {
        Ok(file) => file,
        Err(ref error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("File create error!: {:?}", e),
            },
            others => panic!("File open error!: {:?}", others),
        },
    };
}
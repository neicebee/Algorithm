use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let q = &args[1];
    let f = &args[2];
    println!("검색어: {q}\n대상 파일: {f}");
}
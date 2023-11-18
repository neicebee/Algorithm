use std::{env, process};
use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("인수를 구문분석하는 동안 오류가 발생했습니다: {}", err);
        process::exit(1);
    });
    println!("검색어: {}\n파일 이름: {}", config.query, config.filename);
    
    if let Err(e) = minigrep::run(config) {
        eprintln!("애플리케이션 에러: {}", e);
        process::exit(1);
    }
}
use std::env;
use std::process;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("필요한 인수가 지정되지 않았습니다.");
        }
        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let config = Config::new(&args).unwrap_or_else(
        |err| {
            println!("인수를 구문분석하는 동안 오류가 발생했습니다: {}", err);
            process::exit(1);
        }
    );
    let c = std::fs::read_to_string(&config.filename).unwrap();
    println!("검색어: {}\n대상 파일: {}\n파일 내용:\n{c}", config.query, config.filename);
}
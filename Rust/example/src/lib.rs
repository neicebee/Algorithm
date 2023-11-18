use std::error::Error;
use std::io::Read;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("필요한 인수가 지정되지 않았습니다.");
        }
        Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
            case_sensitive: env::var("CASE_INSENSITIVE").is_err()
        })
    }
}
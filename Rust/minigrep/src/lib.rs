use std::error::Error;
use std::io::Read;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("검색어를 지정해야 합니다."),
        };
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = std::fs::File::open(config.filename)?;
    let mut c = String::new();
    f.read_to_string(&mut c)?;

    let r = if config.case_sensitive {
        search(&config.query, &c)
    } else {
        search_case_insensitive(&config.query, &c)
    };
    
    for l in r {
        println!("{l}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut v = vec![];
    for l in contents.lines() {
        if l.contains(query) {
            v.push(l);
        }
    }
    v
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut v = vec![];
    for l in contents.lines() {
        if l.to_lowercase().contains(&query) {
            v.push(l);
        }
    }
    v
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let q = "duct";
        let c = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        
        assert_eq!(
            vec!["safe, fast, productive."],
            search(q, c)
        );
    }

    #[test]
    fn case_insensitive() {
        let q = "rUsT";
        let c = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(q, c)
        );
    }
}
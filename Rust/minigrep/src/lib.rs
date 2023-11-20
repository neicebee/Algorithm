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
        Ok(Config {
            query: {
                match args.next() {
                    Some(arg) => arg,
                    None => return Err("검색어를 지정해야 합니다."),
                }
            },
            filename: {
                match args.next() {
                    Some(arg) => arg,
                    None => return Err("파일명을 지정해야 합니다."),
                }
            },
            case_sensitive: {
                env::var("CASE_INSENSITIVE").is_err()
            },
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
    contents.lines()
        .filter(|l| l.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|l| l.to_lowercase().contains(&query.to_lowercase()))
        .collect()
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
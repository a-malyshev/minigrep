use std::fs;
use std::error::Error;
use std::env;

pub struct Config{
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    //                               TODO: read about static
    pub fn new(args: &[String]) -> Result<Config,&'static str> {
        if args.len() < 3 {
            return Err("not enough args");
        }
        //FIXME: consider using references with lifetimes
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive: bool = env::var("CASE_INSENSITIVE").is_err();
        println!("value is {} and cs var is {}", 
                 env::var("CASE_INSENSITIVE").unwrap_or_else(|_e| {"env is unset".to_string()}),
                 case_sensitive);
        Ok(Config {query, filename, case_sensitive})
    }
}

//                      TODO: read about trait objects
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {

    let content = fs::read_to_string(&config.filename)?;

    println!("\nsearch results:\n");
    if config.case_sensitive {
        for line in search_case_sensitive(&config.query, &content) {
            println!("{}", line);
        }
    } else {
        for line in search_case_insensitive(&config.query, &content) {
            println!("{}", line);
        }
    }
    Ok(())
}

pub fn search_case_sensitive<'a> (query: &str, content: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            res.push(line.trim());
        }
    }
    res
}

pub fn search_case_insensitive<'a> (query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut res = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line.trim());
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_config_creation() {
        let expected_query = String::from("man");
        let expected_filename = String::from("file");

        let config = Config::new(&[String::from("program_name"),
                    "man".to_string(),"file".to_string()]).unwrap();

        assert_eq!(expected_query, config.query);
        assert_eq!(expected_filename, config.filename);
    }

    #[test]
    fn case_sensitive() {
        let query = "HeaD";
        let content = "
If you can keep your HeaD when all about you   
    Are losing theirs and blaming it on you,";
        
        assert_eq!(vec!["If you can keep your HeaD when all about you"],
                   search_case_sensitive(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "head";
        let content = "
If you can keep your Head when all about you   
    Are losing theirs and blaming it on you,";
        
        assert_eq!(vec!["If you can keep your Head when all about you"],
                   search_case_insensitive(query, content));
    }


}

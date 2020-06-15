use std::fs;
use std::error::Error;

pub struct Config{
    pub query: String,
    pub filename: String,
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
        Ok(Config {query, filename})
    }
}

//                      TODO: read about trait objects
pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {

    let poem = fs::read_to_string(&config.filename)?;
    println!("the content of the file is \n {}", poem);
    Ok(())
}


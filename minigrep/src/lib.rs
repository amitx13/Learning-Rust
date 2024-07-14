use std::fs;
use std::error::Error;

pub fn run(config:Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.filename)?;
    // println!("File Contents :\n{}",content);

    for line in search(&config.query, &content){
        println!("{}",line);
    }
    Ok(())
}

pub struct Config {
    pub query:String,
    pub filename:String,
}

impl Config {
    pub fn new(args:&[String]) -> Result<Config , &str> {
        if args.len() < 3 {
            return Err("not enpugh args");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok(Config{query , filename})
    }
}

pub fn search<'a>(query: &str , contents:& 'a str) ->Vec<&'a str> {
    let mut res = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query){
            res.push(line);
        }
    }
    res
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
fast, safe, productive
Pick three.";

        assert_eq!(vec!["fast, safe, productive"], search(query , contents));
    }

    #[test]
    fn case_sesetive(){
        let query = "rUsT";
        let contents = "\
Rust:
fast, safe, productive
Pick three.
Trust me!";

        assert_eq!(vec!["Rust:","Trust me!"], search(query , contents));
    }

}
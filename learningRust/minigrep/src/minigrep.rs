use std::{error::Error, fs};

pub struct ParserConfig {
    query: String,
    file_path: String,
}

pub fn build(args: Vec<String>) -> Result<ParserConfig, &'static str> {
    if args.len() < 3 {
        return Err("invalid arguments");
    }

    let file_path = args[1].clone();
    let query = args[2].clone();

    Ok(ParserConfig { file_path, query })
}

pub fn run(config: ParserConfig) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for r in search(&config.query, &contents) {
        println!("{r}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a String) -> Vec<&'a str> {
    let mut r: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            r.push(line);
        }
    }

    r
}

#[cfg(test)]
mod test {}

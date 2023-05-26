use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    let contents = fs::read_to_string(config.file_path).expect("Could not read file in path");
}

struct Config<'a> {
    query: &'a String,
    file_path: &'a String,
}

impl Config<'_> {
    fn new(args: &[String]) -> Config {
        let query = &args[1];
        let file_path = &args[2];

        Config { query, file_path }
    }
}

use std::env;

struct Config {
    name: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let name = args[1].clone();

        Ok(Config { name })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    println!("Hello {}!", config.name);
}
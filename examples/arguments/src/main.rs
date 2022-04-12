use std::env;

fn parse_config(args: &[String]) -> &str {
    let name = &args[1];

    name
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let name = parse_config(&args);

    println!("Hello {}!", name);
}
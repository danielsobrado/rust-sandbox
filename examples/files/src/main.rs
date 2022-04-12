use std::fs;

fn main() {
    let filename = "text.txt";
    println!("Reading from file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("Content:\n{}", contents);
}
use std::env;
use std::fs::File;
use std::io::{Read};

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run <file_path>");
        return;
    }

    let file_path = &args[1];
    println!("Reading File: {}", file_path);

    // Read the file contents
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Error opening file: {}", err);
            return;
        }
    };

    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        println!("Error reading file: {}", err);
        return;
    }

    let word_count = count_words(&contents);
    println!("Word Count: {}", word_count);

}

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}
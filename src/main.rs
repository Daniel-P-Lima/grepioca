use std::env; // Use args function
use std::fs; // Handle files

fn main() {
    let args: Vec<String> = env::args().collect(); // collect() turn the iterator into a vector

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path) // Return std::io::Result<String>
        .expect("Should gave been able to read the file");

    println!("With text:\n{contents}");
}

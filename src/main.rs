use std::env; // Use args function

fn main() {
    let args: Vec<String> = env::args().collect(); // collect() turn the iterator into a vector

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("In file {file_path}");
}

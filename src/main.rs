use std::env::args;
use std::fs;

fn main() {
    let args: Vec<String> = args().collect();
    for a in &args {
        println!("{}", a)
    }

    // TODO: Improve input parsing
    let query = &args[1];
    let file_name = &args[2];

    println!("Looking for {query} in {file_name}");

    // TODO: Improve error handling
    let contents = fs::read_to_string(file_name).expect("Should be able to open the file");
    let lines: Vec<&str> = contents.split("\n").collect();

    for l in lines {
        if l.contains(query) {
            println!("{}", l);
        }
    }
}

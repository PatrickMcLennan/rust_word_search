use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::io;

enum State {
    InProgress,
    Found,
    NotFound
}

struct Search {
    word: String,
    count: u8
}

fn search_file(word: Search, file: String) -> () {
    let mut file = File::open(file).expect("The File could not be opened");
    let mut file_contents = String::new();

    file.read_to_string(&mut file_contents).expect("The file could not be read to a string");

    return println!("{}", file_contents);
}

fn main() {
    let mut file_name: String = String::new();
    let mut word_search: Search = Search {
        word: String::new(),
        count: 0
    };

    println!("\nWhat file do you want to read?");
    match io::stdin().read_line(&mut file_name) {
        Ok(_) => println!("You've entered {}", file_name),
        Err(_) => println!("There was an error reading your input")
    }

    println!("What word are you looking for?");
    match io::stdin().read_line(&mut word_search.word) {
        Ok(_) => println!("You've entered {}", word_search.word),
        Err(_) => println!("There was an error reading your input")
    }

    println!("{}", file_name);
    println!("{}", word_search.word);
}
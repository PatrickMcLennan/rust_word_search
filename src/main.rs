use std::fs::File;use std::io::prelude::*;
use std::io;

enum State {
    Found,
    NotFound
}

struct Search {
    word: String,
    count: u8
}

impl Search {
    pub fn add_one(&mut self) -> () {
        self.count += 1
    }
}

fn search_file(search: &mut Search, file_name: String) -> (State, u8) {
    let mut file = match File::open(file_name) {
        Ok(file_exists) => file_exists,
        Err(_error) => panic!("That file does not exist")
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect("The file could not be read to a string");

    let words_vector: Vec<&str> = file_contents.trim().split(' ').collect();

    for word in words_vector {
        if word == search.word {
            search.add_one()
        }
    }

    if search.count == 0 {
        return (State::NotFound, search.count)
    } else {
        return (State::Found, search.count)
    }

}

fn main() {
    let mut file_name: String = String::new();
    let mut word_search: Search = Search {
        word: String::new(),
        count: 0
    };

    println!("What file do you want to read?");
    match io::stdin().read_line(&mut file_name) {
        Ok(_) => println!("You've entered {}", file_name),
        Err(_) => println!("There was an error reading your input")
    }

    println!("What word are you looking for?");
    match io::stdin().read_line(&mut word_search.word) {
        Ok(_) => println!("You've entered {}", word_search.word),
        Err(_) => println!("There was an error reading your input")
    }

    let (state, count) = search_file(&mut word_search, file_name);

    match state {
        State::Found => {
            println!("{} was found {} time(s)", word_search.word, count)
        }
        State::NotFound => {
            println!("{} was not found in the file", word_search.word)
        }
    }
}
// To run via cargo from top-level project directory:
// # with command line arg 
// cargo run --quiet --bin probx_wc -- gettysburg.txt 
// # no command line arg
// cargo run --quiet --bin probx_wc -- 

use std::process::exit;         // bare use of exit(num) function to exit program
use std::env::args;             // bare use of args() function to retrieve commandline args
use std::fs::File;              // for file io
use std::io::{prelude::*, BufReader}; 

// Above import does two things
// 1. Imports everying in std::io::prelude to give access to common IO
//    traits
// 2. Imports BufReader struct which implements line-by-line input
//    processing


// YOUR CODE BELOW for implementing the word count program
fn main() {
    let mut lines = 0;
    let mut words = 0;
    let mut characters = 0;

    let args: Vec<String> = args().collect();

    if args.len() == 1 {
        println!("usage: {} <filename>", args[0]);
        exit(1);
    }

    let file = match File::open(&args[1]) {
        Err(why) => {println!("Couldn't open {}: {}", args[0],why); exit(1)},    
        Ok(file) => file
    };

    let reader = BufReader::new(file);
    let mut linum = 1;
    for line in reader.lines() {
        let text = match line {
            Err(why) => panic!("Couldn't read line {linum}: {}",why),
            Ok(text) => text
        };
        words += project7::prob1_basics::count_words(&text);
        characters += text.len() + 1;
        lines += 1;
        linum += 1;
    }

    println!("{:4} {:4} {:4} {}", lines, words, characters, args[1]);

}

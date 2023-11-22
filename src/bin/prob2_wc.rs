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

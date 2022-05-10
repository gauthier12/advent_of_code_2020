#[macro_use]
extern crate scan_fmt;
extern crate regex;
use rayon::prelude::*;
use regex::Regex;
use std::env;
use std::fs;
use std::time::Instant;


fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let filename = &args[1];
   
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    for line in contents.lines() {
        println!("Line : {}", line);
    }

    let duration = start.elapsed();

    println!("Time elapsed in total is: {:?}", duration);
}

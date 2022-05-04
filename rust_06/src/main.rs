extern crate ndarray;
#[macro_use]
extern crate scan_fmt;
use rayon::prelude::*;
use std::env;
use std::fs;
use std::time::Instant;
#[derive(Debug)]
struct Form {
    answers: [u32; 26],
    size: u32,
}
fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    let mut cur_form = Form {
        answers: [0; 26],
        size: 0,
    };
    let mut form_db: Vec<Form> = Vec::new();
    println!("{:?}", args);
    let filename = &args[1];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    for line in contents.lines() {
        if line.split_whitespace().count() > 0 {
            for c in line.chars() {
                cur_form.answers[c as usize - 97] += 1;
            }
            cur_form.size += 1;
        } else {
            // new group
            //println!("cur group {:?}", cur_form);
            form_db.push(cur_form);
            cur_form = Form {
                answers: [0; 26],
                size: 0,
            };
        }
    }
    form_db.push(cur_form);
    let num_any_yes: u32 = form_db
        .iter()
        .map(|cf| {
            cf.answers
                .iter()
                .map(|&ccf| if ccf > 0 { 1 } else { 0 })
                .sum::<u32>()
        })
        .sum();
    let num_all_yes: u32 = form_db
        .iter()
        .map(|cf| {
            cf.answers
                .iter()
                .map(|&ccf| if ccf == cf.size as u32 { 1 } else { 0 })
                .sum::<u32>()
        })
        .sum();
    println!("Number of any yes answers : {:?}", num_any_yes);
    println!("Number of all yes answers : {:?}", num_all_yes);
    let duration = start.elapsed();
    println!("Time elapsed in total is: {:?}", duration);
}

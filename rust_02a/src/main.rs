use std::env;
use std::fs;
use rayon::prelude::*;
use std::time::{Instant};
#[derive(Debug)]
struct Rule {
    min_repetition: u32,
    max_repetition: u32,
    letter: char,
    word: String,
}

fn check_rule(rule : &Rule) -> bool {
    //println!("cur rule {:?}",rule);
    let num_repetition :u32 = rule.word.chars().map(|c| (c==rule.letter) as u32 ).sum();
    num_repetition >= rule.min_repetition && num_repetition <= rule.max_repetition
}

fn main() {

    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let filename = &args[1];
    println!("In file {}", filename);
    println!("current_num_threads() = {}", rayon::current_num_threads());
    match rayon::current_thread_index() {
        // The division was valid
        Some(nt) => println!("current_thread_index() = {}", nt),
        None    => println!("No thread index"),
    }

if let Ok(contents) = fs::read_to_string(filename)
{

    let mut rule_list : Vec<Rule> = Vec::new();
    for cur_line in contents.lines() {
            //.parse::<i32>().expect("is not a number"));
            let cur_rule : Vec<_> = cur_line.split(|c| c == ':' || c == ' ' || c == '-').collect();
            rule_list.push(
                 Rule {
                    min_repetition : cur_rule[0].parse::<u32>().expect("wrong value"),
                    max_repetition : cur_rule[1].parse::<u32>().expect("wrong value"),
                    letter : cur_rule[2].parse::<char>().expect("wrong value"),
                    word : cur_rule[4].parse::<String>().expect("wrong value")
                     }
            );
    }
    println!("Methode A");
    let start_a = Instant::now();
    let mut tot_a : u32 =0;
    for cur_rule in rule_list.iter() {
        tot_a += check_rule(&cur_rule) as u32;
    }
    let duration_a = start_a.elapsed();
    println!("Methode B");
    let start_b = Instant::now();
    let tot_b:u32 = rule_list.iter().map(|cr| (check_rule(&cr) as u32) ).sum();
    let duration_b = start_b.elapsed();

    println!("Methode C");
    let start_c = Instant::now();
    let tot_c:u32 = rule_list.par_iter().map(|cr| (check_rule(&cr) as u32) ).sum();
    let duration_c = start_c.elapsed();
    println!("Time elapsed in A() is: {:?} result {:}", duration_a,tot_a);
    println!("Time elapsed in B() is: {:?} result {:}", duration_b,tot_b);
    println!("Time elapsed in C() is: {:?} result {:}", duration_c,tot_c);
}
else
{
    println!("Something went wrong")
}

let duration = start.elapsed();

println!("Time elapsed in total is: {:?}", duration);
}

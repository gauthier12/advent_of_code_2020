use rayon::prelude::*;
use std::env;
use std::fs;
use std::time::Instant;
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
        None => println!("No thread index"),
    }

    if let Ok(contents) = fs::read_to_string(filename) {
        let mut numbers: Vec<i32> = Vec::new();
        for cur_line_a in contents.lines() {
            numbers.push(cur_line_a.parse::<i32>().expect("is not a number"));
        }

        println!("Methode A");
        let start_a = Instant::now();
        'solve: for cur_line_a in numbers.iter() {
            let number_a = cur_line_a;
            for cur_line_b in numbers.iter() {
                let number_b = cur_line_b;
                let sum = number_a + number_b;
                //println!("{} + {} = {}", number_a,number_b,sum);
                if sum == 2020 {
                    println!("   solution : {}", number_a * number_b);
                    break 'solve;
                }
            }
        }

        let duration_a = start_a.elapsed();
        println!("Methode B");
        let start_b = Instant::now();
        numbers.iter().any(|&vala| {
            numbers.iter().any(|&valb| {
                //println!("{} + {} = {}", vala,valb,vala + valb);

                if vala + valb == 2020 {
                    println!("   solution : {}", vala * valb);
                    true
                } else {
                    false
                }
            })
        });
        let duration_b = start_b.elapsed();

        println!("Methode C");
        let start_c = Instant::now();
        numbers.par_iter().any(|&vala| {
            numbers.iter().any(|&valb| {
                //println!("{} + {} = {}", vala,valb,vala + valb);
                /*match rayon::current_thread_index() {
                    // The division was valid
                    Some(nt) => println!("current_thread_index() = {}", nt),
                    None    => println!("No thread index"),
                }*/
                if vala + valb == 2020 {
                    println!("   solution : {}", vala * valb);
                    true
                } else {
                    false
                }
            })
        });

        let duration_c = start_c.elapsed();
        println!("Time elapsed in A() is: {:?}", duration_a);
        println!("Time elapsed in B() is: {:?}", duration_b);
        println!("Time elapsed in C() is: {:?}", duration_c);
    } else {
        println!("Something went wrong")
    }

    let duration = start.elapsed();

    println!("Time elapsed in total is: {:?}", duration);
}

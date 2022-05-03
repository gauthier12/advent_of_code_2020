extern crate ndarray;
#[macro_use]
extern crate scan_fmt;
use rayon::prelude::*;
use std::env;
use std::fs;
use std::time::Instant;
#[derive(Debug)]
struct Passeport {
    byr: Option<u32>,
    iyr: Option<u32>,
    eyr: Option<u32>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<u32>,
    num_info: u32,
}
fn init_passport() -> Passeport
{
    Passeport {
        byr: None,
        iyr: None,
        eyr: None,
        hgt: None,
        hcl: None,
        ecl: None,
        pid: None,
        cid: None,
        num_info: 0,
    }
}
fn check_passport(passport: &Passeport) -> bool{
    false
}

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let filename = &args[1];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut num_valid: u32 = 0;
    let mut cur_passport: Passeport;
    cur_passport = init_passport();
    for line in contents.lines() {
        if line.split_whitespace().count() > 0 {
            for s in line.split_whitespace() {
                if let Ok((str1, str2)) = scan_fmt!(
                    s,       // input string
                    "{}:{}", // format
                    String, String
                ) {
                    match str1.as_ref() {
                        "byr" => {
                            cur_passport.byr =
                                Some(str2.parse::<u32>().expect("Wrong integer format"))
                        }
                        "iyr" => {
                            cur_passport.iyr =
                                Some(str2.parse::<u32>().expect("Wrong integer format"))
                        }
                        "eyr" => {
                            cur_passport.eyr =
                                Some(str2.parse::<u32>().expect("Wrong integer format"))
                        }
                        "hgt" => cur_passport.hgt = Some(str2),
                        "hcl" => cur_passport.hcl = Some(str2),
                        "ecl" => cur_passport.ecl = Some(str2),
                        "pid" => cur_passport.pid = Some(str2),
                        "cid" => {
                            cur_passport.cid =
                                Some(str2.parse::<u32>().expect("Wrong integer format"))
                        }
                        _ => (),
                    }
                    if str1 != "cid" {
                        cur_passport.num_info += 1;
                        if cur_passport.num_info >= 7 {
                            num_valid += 1;
                        }
                    }
                } else {
                    panic!("Parsing error")
                }
            }
        } else {
            cur_passport = init_passport()
        }
    }
    println!("Number of valid passport : {:?}", num_valid);
    let duration = start.elapsed();
    println!("Time elapsed in total is: {:?}", duration);
}

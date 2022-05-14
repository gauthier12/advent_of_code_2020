#[macro_use]
extern crate scan_fmt;
extern crate regex;

use rayon::prelude::*;
use regex::Regex;
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
fn init_passport() -> Passeport {
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
fn check_passport(passport: &Passeport) -> bool {
    //byr (Birth Year) - four digits; at least 1920 and at most 2002.
    if let Some(c_byr) = passport.byr {
        if c_byr < 1920 || c_byr > 2002 {
            return false;
        }
    }
    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    if let Some(c_iyr) = passport.iyr {
        if c_iyr < 2010 || c_iyr > 2020 {
            return false;
        }
    }
    //eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    if let Some(c_eyr) = passport.eyr {
        if c_eyr < 2020 || c_eyr > 2030 {
            return false;
        }
    }
    //ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    if let Some(ecl_string) = &passport.ecl {
        match ecl_string.as_ref() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {}
            _ => return false,
        }
    } else {
        return false;
    }
    //hgt (Height) - a number followed by either cm or in:
    if let Some(height_string) = &passport.hgt {
        let len = height_string.len();
        if let Ok(heightvalue) = height_string[..len - 2].parse::<u32>() {
            let heightunit = &height_string[len - 2..];
            match heightunit {
                "cm" => {
                    //If cm, the number must be at least 150 and at most 193
                    if heightvalue < 150 || heightvalue > 193 {
                        return false;
                    }
                }
                "in" => {
                    //    If in, the number must be at least 59 and at most 76.
                    if heightvalue < 59 || heightvalue > 76 {
                        return false;
                    }
                }
                _ => return false,
            };
        } else {
            return false;
        }
    } else {
        return false;
    }
    //hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    if let Some(hcl_string) = &passport.hcl {
        let re_string = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
        if !re_string.is_match(hcl_string.as_ref()) {
            return false;
        }
    } else {
        return false;
    }
    //pid (Passport ID) - a nine-digit number, including leading zeroes.
    if let Some(pid_string) = &passport.pid {
        if pid_string.len() != 9 {
            return false;
        } else {
            let re_pid = Regex::new(r"^\d{9}$").unwrap();
            if !re_pid.is_match(pid_string.as_ref()) {
                return false;
            }
        }
    } else {
        return false;
    }
    //cid (Country ID) - ignored, missing or not.
    true
}

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let filename = &args[1];
    let mut passport_db: Vec<Passeport> = Vec::new();
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut cur_passport: Passeport;
    cur_passport = init_passport();
    for line in contents.lines() {
        if line.split_whitespace().count() > 0 {
            for s in line.split_whitespace() {
                if let Ok((str1, str2)) = scan_fmt!(s, "{}:{}", String, String) {
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
                    }
                } else {
                    panic!("Parsing error")
                }
            }
        } else {
            if cur_passport.num_info >= 7 {
                passport_db.push(cur_passport);
            }
            cur_passport = init_passport();
        }
    }
    if cur_passport.num_info >= 7 {
        passport_db.push(cur_passport);
    }
    //num_valid += check_passport(&cur_passport) as u32;
    let num_valid: u8 = passport_db
        .par_iter()
        .map(|cp| check_passport(cp) as u8)
        .sum();
    println!("Number of valid passport : {:?}", num_valid);
    let duration = start.elapsed();
    println!("Time elapsed in total is: {:?}", duration);
}
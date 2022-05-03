extern crate ndarray;
#[macro_use]
extern crate scan_fmt;

use rayon::prelude::*;
use std::env;
use std::fs;
use std::time::Instant;
#[derive(Debug)]
struct Seat
{
    vpos:u32,
    hpos:u32
}
fn seatId(seat:&Seat) ->u32
{
    seat.vpos * 8 + seat.hpos
}
fn convert_to_num(str1:&str) -> u32 {
    let mut tot=0;
    for ichar in str1.chars()
    {
        tot=tot*2+ if ichar == 'F' ||ichar == 'L'  {0} else {1}
    }
    tot
}

fn main() {
    let mut seat_occupation: [bool;1024] = [false;1024] ;
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    let mut seat_db:Vec<Seat> = Vec::new();
    println!("{:?}", args);
    let filename = &args[1];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    for line in contents.lines() {
        let s_vpos = &line[..7];
        let s_hpos = &line[7..];
        let n_vpos = convert_to_num(s_vpos);
        let n_hpos = convert_to_num(s_hpos);
        seat_db.push(Seat{vpos:n_vpos,hpos:n_hpos});

        //Every seat also has a unique seat ID: multiply the row by 8, then add the column. In this example, the seat has ID
        let seat_id = seatId(seat_db.last().unwrap());
        seat_occupation[ seat_id as usize] = true;
    }
    if let Some(max_id) = seat_db.iter().map(|cs| seatId(cs)).max()
    {
        println!("Max ID :{}", max_id);
        for (idx,c_seat) in seat_occupation.iter().enumerate()
        {
            if ! c_seat && idx > 7 && idx < max_id as usize
            {
                println!("My seat  : {:?}", idx);
            }
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed in total is: {:?}", duration);
}

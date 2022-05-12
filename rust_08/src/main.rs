#[macro_use]
extern crate scan_fmt;
use std::env;
use std::fs;
use std::time::Instant;
#[derive(Debug)]
enum EndReason {
    LoopDetected,
    EndPrg,
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum Cmd {
    Acc,
    Jmp,
    Nop,
}
#[derive(Debug, Clone)]
struct Instruction {
    cmd: Cmd,
    value: i32,
}
fn run_program(prg: &Vec<Instruction>) -> Result<(usize, i32, EndReason), ()> {
    let mut n_ins = 0;
    let mut i_ins = 0;
    let mut accumulator: i32 = 0;
    let n_ins_max = prg.len();
    let mut run = vec![0; n_ins_max];
    while n_ins < n_ins_max {
        if i_ins >= n_ins_max {
            return Ok((i_ins, accumulator, EndReason::EndPrg));
        }
        if run[i_ins] > 0 {
            return Ok((i_ins, accumulator, EndReason::LoopDetected));
        }
        run[i_ins] += 1;
        match prg[i_ins].cmd {
            Cmd::Acc => {
                accumulator += prg[i_ins].value;
                i_ins = i_ins + 1;
            }
            Cmd::Jmp => {
                i_ins = (i_ins as i32 + prg[i_ins].value) as usize;
            }
            Cmd::Nop => {
                i_ins = i_ins + 1;
            }
        }
        n_ins += 1;
    }
    return Err(());
}
fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let filename = &args[1];
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut program: Vec<Instruction> = Vec::new();
    for line in contents.lines() {
        if let Ok((cmd, value)) = scan_fmt!(line, "{} {}", String, i32) {
            match cmd.as_ref() {
                "acc" => program.push(Instruction {
                    cmd: Cmd::Acc,
                    value,
                }),
                "jmp" => program.push(Instruction {
                    cmd: Cmd::Jmp,
                    value,
                }),
                "nop" => program.push(Instruction {
                    cmd: Cmd::Nop,
                    value,
                }),
                _ => {}
            }
        }
    }
    println!("Part 1");
    match run_program(&program) {
        Ok((final_ins, accumulator, EndReason::LoopDetected)) => {
            println!(
                "Loop detected at instruction {}. Accumulator value {}",
                final_ins, accumulator
            );
        }
        _ => {
            println!("Error executing the program");
        }
    }

    println!("Part 2 ");
    for i_ins in 0..program.len() {
        if program[i_ins].cmd == Cmd::Nop || program[i_ins].cmd == Cmd::Jmp {
            let mut program2 = program.clone();
            program2[i_ins].cmd = if program[i_ins].cmd == Cmd::Nop {
                Cmd::Jmp
            } else {
                Cmd::Nop
            };
            match run_program(&program2) {
                Ok((final_ins, accumulator, EndReason::EndPrg)) => {
                    println!(
                        "End program detected at instruction {}. Accumulator value {}",
                        final_ins, accumulator
                    );
                    break;
                }
                _ => {}
            }
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed in total is: {:?}", duration);
}

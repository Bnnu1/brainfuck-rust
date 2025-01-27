use std::fs;
use std::io;
use std::io::Write;

fn main() {
    let mut tape: Vec<u8> = vec![0; 30000];
    let mut tape_ptr: usize = 0;

    let mut loops: Vec<i32> = vec![];

    let mut filename: String = String::new();
    io::stdin().read_line(&mut filename);

    let mut code = fs::read_to_string(filename.trim())
        .expect("Should have been able to read the file");
    let mut i: i32 = 0;
    while i + 1 < code.len() as i32 { 
        match code.chars().nth(i as usize).unwrap() {
            '<' => tape_ptr -= 1,
            '>' => tape_ptr += 1,
            '-' => tape[tape_ptr] -= 1,
            '+' => tape[tape_ptr] += 1,
            '.' => print!("{}", tape[tape_ptr] as char),
            ',' => {
                    let mut input: String = String::new();
                    io::stdin().read_line(&mut input);
                    tape[tape_ptr] = input.chars().nth(0).unwrap() as u8;
            }
            '[' => loops.push(i),
            ']' => {
                if tape[tape_ptr] != 0 {
                    i = *loops.last().unwrap();
                } else {
                    loops.pop();
                }
            } 
            _ => { i += 1; continue; }
        }
        i += 1;
    }

    println!();
}

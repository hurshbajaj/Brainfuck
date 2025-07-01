use std::env;
use std::fs;
use std::io::{self, Read};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: brainfuck <filename.bf>");
        return;
    }

    let filename = &args[1];
    let source = fs::read_to_string(filename).expect("Could not read file");

    run(&source);
}

fn run(code: &str) {
    let mut tape = vec![0u8; 30_000];
    let mut ptr = 0;
    let code_chars: Vec<char> = code
        .chars()
        .filter(|c| matches!(c, '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']'))
        .collect();
    let mut pc = 0;
    let mut loop_stack = Vec::new();

    while pc < code_chars.len() {
        match code_chars[pc] {
            '>' => ptr += 1,
            '<' => ptr -= 1,
            '+' => tape[ptr] = tape[ptr].wrapping_add(1),
            '-' => tape[ptr] = tape[ptr].wrapping_sub(1),
            '.' => print!("{}", tape[ptr] as char),
            ',' => {
                let mut input = [0];
                if io::stdin().read_exact(&mut input).is_ok() {
                    tape[ptr] = input[0];
                }
            }
            '[' => {
                loop_stack.push(pc);
            }
            ']' => {
                if tape[ptr] != 0 {
                    pc = *loop_stack.last().expect("Unmatched ]");
                    continue;
                } else {
                    loop_stack.pop();
                }
            }
            _ => {}
        }

        pc += 1;
    }
}


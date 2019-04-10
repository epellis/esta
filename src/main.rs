use std::io;
use std::io::Write;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => eprintln!("Usage: esta [source]"),
    }
}

fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Couldn't read input");
        line.trim();

        esta::run(&line);

        io::stdout().flush().unwrap();
    }
}

fn run_file(path: &str) {
    let buffer = fs::read_to_string(path)
        .expect("Couldn't read file!");
    esta::run(&buffer);
}

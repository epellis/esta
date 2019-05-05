use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::process;

extern crate env_logger;
extern crate log;

fn main() {
    env_logger::builder().default_format_timestamp(false).init();

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
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Couldn't read input");

        match esta::run(&buffer) {
            Ok(()) => {}
            Err(why) => eprintln!("{}", why),
        };

        io::stdout().flush().unwrap();
    }
}

fn run_file(path: &str) {
    let buffer = fs::read_to_string(path).expect("Couldn't read file!");
    let result = match esta::run(&buffer) {
        Ok(()) => 0,
        Err(why) => {
            eprintln!("{}", why);
            1
        }
    };
    process::exit(result);
}

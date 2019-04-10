mod scanner;
mod tokens;

pub fn run(input: &str) {
    let tokens = scanner::scan(input);
    println!("Scanned Tokens: {:?}", tokens);
}

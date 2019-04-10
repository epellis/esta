pub mod frontend;

#[macro_use] extern crate lazy_static;
extern crate regex;


pub fn run(input: &str) {
    frontend::run(input);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

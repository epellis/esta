mod ast;
pub mod frontend;

#[macro_use]
extern crate lalrpop_util;

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

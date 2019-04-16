mod ast;
pub mod frontend;

#[macro_use]
extern crate lalrpop_util;

pub fn run(input: &str) -> Result<(), &'static str> {
    frontend::run(input)
}

#[cfg(test)]
mod tests {
    use crate::frontend;

    #[test]
    fn test_var() {
        let input = "var a;";
        let result = frontend::run(input);
        assert_eq!(result.is_ok(), true);

        // Err because there is no semicolon
        let input = "var a";
        let result = frontend::run(input);
        assert_eq!(result.is_err(), true);

        let input = "var a: num;";
        let result = frontend::run(input);
        assert_eq!(result.is_ok(), true);

        let input = "var a = 1;";
        let result = frontend::run(input);
        assert_eq!(result.is_ok(), true);

        let input = "var a: num = 1;";
        let result = frontend::run(input);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_assign() {
        let input = "var a = 1;";
        let result = frontend::run(input);
        assert_eq!(result.is_ok(), true);

        // Err because b is not initialized
        let input = "var a = b;";
        let result = frontend::run(input);
        assert_eq!(result.is_err(), true);

        let input = "var a = 1 + 2;";
        let result = frontend::run(input);
        assert_eq!(result.is_ok(), true);

        // Err because a is not assigned
        let input = "a = 2;";
        let result = frontend::run(input);
        assert_eq!(result.is_err(), true);

        let input = "var a; a = 2;";
        let result = frontend::run(input);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_op() {
        let input = "var a = 1 + 1;";
        let result = frontend::run(input);
        assert_eq!(result.is_ok(), true);

        let input = "var a = 1 * 1 + 2 / 3 - 4;";
        let result = frontend::run(input);
        assert_eq!(result.is_ok(), true);

        let input = "var a = True or False;";
        let result = frontend::run(input);
        assert_eq!(result.is_ok(), true);

        let input = "var a = True and False;";
        let result = frontend::run(input);
        assert_eq!(result.is_ok(), true);

        let input = "var a = 1 == 2;";
        let result = frontend::run(input);
        assert_eq!(result.is_ok(), true);

        let input = "var a = 1 != 2;";
        let result = frontend::run(input);
        assert_eq!(result.is_ok(), true);

        let input = "var a = 1 < 2;";
        let result = frontend::run(input);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_decl() {
        let input = "fun foo() {}";
        let result = frontend::run(input);
        assert_eq!(result.is_ok(), true);

        let input = "fun foo(a) {}";
        let result = frontend::run(input);
        assert_eq!(result.is_ok(), true);

        let input = "fun foo(a: num) {}";
        let result = frontend::run(input);
        assert_eq!(result.is_ok(), true);

        let input = "fun foo(a, b) {}";
        let result = frontend::run(input);
        assert_eq!(result.is_ok(), true);

        let input = "fun foo(a: num, b: nil) {}";
        let result = frontend::run(input);
        assert_eq!(result.is_ok(), true);

        let input = "fun foo() -> nil {}";
        let result = frontend::run(input);
        assert_eq!(result.is_ok(), true);

        let input = "fun foo(a) { return a; } ";
        let result = frontend::run(input);
        assert_eq!(result.is_ok(), true);

        let input = "fun foo(a: num, b: nil) -> num { return a * b; }";
        let result = frontend::run(input);
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn test_control() {
        let input = "for var i = 0; i < 4; i = i + 1; { }";
        let result = frontend::run(input);
        assert_eq!(result.is_ok(), true);

        let input = "var i; while i != 0 { i = 0; }";
        let result = frontend::run(input);
        assert_eq!(result.is_ok(), true);

        let input = "if False { } ";
        let result = frontend::run(input);
        assert_eq!(result.is_ok(), true);
    }
}

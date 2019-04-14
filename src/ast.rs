use std::fmt;

/// Expression Abstract Syntax Tree Node
///
/// Expressions are a structure of values that can be evaluated by the
/// interpreter to yield a result.
pub enum Expr {
    Identifier(String),
    Literal(Literal),
    BinaryOp(Box<Expr>, Opcode, Box<Expr>),
}

pub enum Literal {
    Number(i32),
    Boolean(bool),
    String(String),
}

pub enum Opcode {
    // Mathematics
    Add,
    Sub,
    Mul,
    Div,
    // Boolean
    Greater,
    GreaterEq,
    Lesser,
    LesserEq,
    EqualEqual,
    BangEqual,
    // Logical
    And,
    Or,
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Identifier(identity) => write!(f, "{}", identity),
            Expr::Literal(literal) => write!(f, "{}", literal),
            Expr::BinaryOp(left, opcode, right) => write!(f, "({} {} {})", opcode, left, right),
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Number(number) => write!(f, "{}", number),
            Literal::Boolean(boolean) => write!(f, "{}", boolean),
            Literal::String(string) => write!(f, "{}", string),
        }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Opcode::Add => write!(f, "+"),
            Opcode::Sub => write!(f, "-"),
            Opcode::Mul => write!(f, "*"),
            Opcode::Div => write!(f, "/"),
            Opcode::Greater => write!(f, ">"),
            Opcode::GreaterEq => write!(f, ">="),
            Opcode::Lesser => write!(f, "<"),
            Opcode::LesserEq => write!(f, "<="),
            Opcode::EqualEqual => write!(f, "=="),
            Opcode::BangEqual => write!(f, "!="),
            Opcode::And => write!(f, "and"),
            Opcode::Or => write!(f, "or"),
        }
    }
}

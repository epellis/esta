use std::fmt;

/// Statement Abstract Syntax Tree Node
/// - Declaration: Define an identifier and bind it to the expression
/// - Assignment: Binds an identifier to a new expression
#[derive(Clone)]
pub enum Stmt {
    Block(Vec<Box<Stmt>>),
    If(Box<Expr>, Box<Stmt>, Box<Stmt>),
    While(Box<Expr>, Box<Stmt>),
    For(
        Option<Box<Expr>>,
        Option<Box<Expr>>,
        Option<Box<Expr>>,
        Box<Stmt>,
    ),
    Return(Option<Box<Expr>>),
    Break,
    Continue,
    Declaration(String, Box<Expr>),
    FunDecl(String, Vec<String>, Box<Stmt>),
    Assignment(Box<Expr>, Box<Expr>),
    ImpureCall(Box<Expr>),
}

/// Expression Abstract Syntax Tree Node
///
/// Expressions are a structure of values that can be evaluated by the
/// interpreter to yield a result.
///
/// A few variants are viewed in detail below:
/// - Identifier: Any token that matches [[:alpha:]]\w*
/// - Literal: See below
/// - BinaryOp: A binary operation performed on a lhs and a rhs
/// - UnaryOp: A unary operation performed on a rhs
#[derive(Clone)]
pub enum Expr {
    Identifier(String),
    Literal(Literal),
    BinaryOp(Box<Expr>, Opcode, Box<Expr>),
    UnaryOp(Opcode, Box<Expr>),
    FunCall(String, Vec<Box<Expr>>),
}

/// Literal Values
///
/// Literal Enums are values that are known at compile time
#[derive(Clone)]
pub enum Literal {
    Number(i32),
    Boolean(bool),
    String(String),
    Nil,
}

/// Opcodes
///
/// Opcodes are fundamental operations on (usually) two operands
#[derive(Clone)]
pub enum Opcode {
    // Mathematics
    Add,
    Sub,
    Mul,
    Div,
    // Boolean
    Greater,
    GreaterEqual,
    Lesser,
    LesserEqual,
    EqualEqual,
    BangEqual,
    // Logical
    And,
    Or,
    Not,
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::Block(stmts) => {
                let stmts: Vec<String> = stmts.iter().map(|stmt| format!("{}", stmt)).collect();
                let stmts = stmts.join(", ");
                write!(f, "([{}])", stmts)
            }
            Stmt::Assignment(lhs, rhs) => write!(f, "({} <- {})", lhs, rhs),
            Stmt::Declaration(identifier, binding) => {
                write!(f, "(define {} {})", identifier, binding)
            }
            Stmt::While(condition, block) => write!(f, "(while {} {})", condition, block),
            Stmt::If(condition, block, alternate) => {
                write!(f, "(if {} {} {})", condition, block, alternate)
            }
            Stmt::For(setup, test, increment, block) => {
                let setup = setup
                    .clone()
                    .unwrap_or(Box::new(Expr::Literal(Literal::Nil)));
                let test = test
                    .clone()
                    .unwrap_or(Box::new(Expr::Literal(Literal::Nil)));
                let increment = increment
                    .clone()
                    .unwrap_or(Box::new(Expr::Literal(Literal::Nil)));
                write!(f, "(for {} {} {} {})", setup, test, increment, block)
            }
            Stmt::Return(returned) => {
                let returned = returned
                    .clone()
                    .unwrap_or(Box::new(Expr::Literal(Literal::Nil)));
                write!(f, "(return {})", returned)
            }
            Stmt::Break => write!(f, "(break)"),
            Stmt::Continue => write!(f, "(continue)"),
            Stmt::FunDecl(name, params, body) => write!(f, "(fun {} {:?} {})", name, params, body),
            Stmt::ImpureCall(fun) => write!(f, "{}", fun),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Identifier(identity) => write!(f, "{}", identity),
            Expr::Literal(literal) => write!(f, "{}", literal),
            Expr::BinaryOp(left, opcode, right) => write!(f, "({} {} {})", opcode, left, right),
            Expr::UnaryOp(opcode, right) => write!(f, "({} {})", opcode, right),
            Expr::FunCall(name, args) => {
                let args: Vec<String> = args.iter().map(|stmt| format!("{}", stmt)).collect();
                let args = args.join(", ");
                write!(f, "({}, {})", name, args)
            }
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Literal::Number(number) => write!(f, "{}", number),
            Literal::Boolean(boolean) => write!(f, "{}", boolean),
            Literal::String(string) => write!(f, "{}", string),
            Literal::Nil => write!(f, "Nil"),
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
            Opcode::GreaterEqual => write!(f, ">="),
            Opcode::Lesser => write!(f, "<"),
            Opcode::LesserEqual => write!(f, "<="),
            Opcode::EqualEqual => write!(f, "=="),
            Opcode::BangEqual => write!(f, "!="),
            Opcode::And => write!(f, "and"),
            Opcode::Or => write!(f, "or"),
            Opcode::Not => write!(f, "not"),
        }
    }
}

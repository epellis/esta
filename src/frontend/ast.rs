use std::fmt;

/// Statement Abstract Parse Tree Node
/// - Declaration: Define an identifier and bind it to the expression
/// - Assignment: Binds an identifier to a new expression
#[derive(Clone)]
pub enum Stmt {
    Block(Vec<Box<Stmt>>),
    If(ExprNode, Box<Stmt>, Box<Stmt>),
    While(ExprNode, Box<Stmt>),
    For(
        Option<Box<Stmt>>,
        Option<ExprNode>,
        Option<Box<Stmt>>,
        Box<Stmt>,
    ),
    Return(Option<ExprNode>),
    Declaration(String, ExprNode),
    FunDecl(String, Vec<ExprNode>, Type, Box<Stmt>),
    Assignment(ExprNode, ExprNode),
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
pub struct ExprNode {
    pub expr: Box<Expr>,
    pub type_of: Type,
}

#[derive(Clone)]
pub enum Expr {
    Identifier(String),
    // TODO: Assign type
    Literal(Literal),
    BinaryOp(ExprNode, Opcode, ExprNode),
    UnaryOp(Opcode, ExprNode),
    FunCall(String, Vec<ExprNode>),
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

/// Basic Types
#[derive(Clone)]
pub enum Type {
    Number,
    Boolean,
    String,
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

impl ExprNode {
    pub fn new_default() -> ExprNode {
        ExprNode {
            expr: Box::new(Expr::Literal(Literal::Nil)),
            type_of: Type::Nil,
        }
    }
    /// Create a new expression with an indeterminate type
    ///
    /// This function will attempt to find the type by searching it's children
    pub fn new_untyped(expr: Expr) -> ExprNode {
        //        let type_of = expr.type_of();
        let type_of = Type::Nil;
        ExprNode {
            expr: Box::new(expr),
            type_of,
        }
    }
    pub fn new_typed(expr: Expr, type_of: Type) -> ExprNode {
        ExprNode {
            expr: Box::new(expr),
            type_of,
        }
    }
    pub fn new_nil() -> ExprNode {
        ExprNode {
            expr: Box::new(Expr::Literal(Literal::Nil)),
            type_of: Type::Nil,
        }
    }
    pub fn identify(&self) -> String {
        match &*self.expr {
            Expr::Identifier(identity) => identity.clone(),
            _ => "".to_string(),
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Stmt::Block(stmts) => {
                let stmts: Vec<String> = stmts.iter().map(|stmt| format!("{}", stmt)).collect();
                let stmts = stmts.join("\n");
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
                let setup = setup.clone().unwrap_or(Box::new(Stmt::Block(Vec::new())));
                let test = test.clone().unwrap_or(ExprNode::new_default());
                let increment = increment
                    .clone()
                    .unwrap_or(Box::new(Stmt::Block(Vec::new())));
                write!(f, "(for {} {} {} {})", setup, test, increment, block)
            }
            Stmt::Return(returned) => {
                let returned = returned.clone().unwrap_or(ExprNode::new_default());
                write!(f, "(return {})", returned)
            }
            //            Stmt::Break => write!(f, "(break)"),
            //            Stmt::Continue => write!(f, "(continue)"),
            Stmt::FunDecl(name, params, ret, body) => {
                let params: Vec<String> = params.iter().map(|p| format!("{}", p)).collect();
                let params = params.join(", ");
                write!(f, "(fun {} {}-> {} ({}))", name, params, ret, body)
            }
        }
    }
}

impl fmt::Display for ExprNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.expr, self.type_of)
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Nil => write!(f, "nil"),
            Type::Number => write!(f, "num"),
            Type::Boolean => write!(f, "bool"),
            Type::String => write!(f, "str"),
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

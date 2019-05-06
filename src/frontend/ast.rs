/// Statement AST
///     Block: Scoped Block
///     FlatBlock: Unscoped Block
/// ...
#[derive(Debug, Clone)]
pub enum Stmt {
    Block(Vec<Box<Stmt>>),
    FlatBlock(Vec<Box<Stmt>>),
    If(Box<Expr>, Box<Stmt>, Box<Stmt>),
    While(Box<Expr>, Box<Stmt>),
    Return(Option<Box<Expr>>),
    Declaration(Identifier),
    FunDecl(Identifier, Vec<Identifier>, Box<Stmt>),
    Assignment(Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Id(Identifier),
    Literal(Literal),
    BinaryOp(Box<Expr>, Opcode, Box<Expr>),
    UnaryOp(Opcode, Box<Expr>),
    FunCall(String, Vec<Expr>),
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub id: String,
    pub type_of: String,
}

// TODO: Only need to annotate type if it is an identifier
impl Identifier {
    pub fn new(id: String) -> Self {
        Identifier {
            id,
            type_of: "".to_string(),
        }
    }
    pub fn new_typed(id: String, type_of: String) -> Self {
        Identifier { id, type_of }
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(i64),
    Boolean(bool),
    String(String),
    Nil,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Opcode {
    // Mathematics
    Add,
    Sub,
    Mul,
    Div,
    Mod,
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

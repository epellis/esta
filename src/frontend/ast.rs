use std::collections::HashMap;

/// Statement AST
///     Block: Scoped Block
///     FlatBlock: Unscoped Block
/// ...
#[derive(Debug, Clone)]
pub enum Stmt {
    Block(Vec<Box<Stmt>>, bool),
    If(Box<Expr>, Box<Stmt>, Box<Stmt>),
    While(Box<Expr>, Box<Stmt>),
    Return(Option<Box<Expr>>),
    Declaration(Identifier),
    FunDecl(Identifier, Vec<Identifier>, Box<Stmt>),
    Assignment(Box<Expr>, Box<Expr>),
    Struct(String, Vec<Identifier>),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Id(Identifier),
    Dot(Identifier, Box<Expr>),
    Literal(Literal),
    List(Vec<Box<Expr>>),
    BinaryOp(Box<Expr>, Opcode, Box<Expr>),
    UnaryOp(Opcode, Box<Expr>),
    FunCall(String, Vec<Expr>),
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub id: String,
    pub type_of: String,
}

impl Identifier {
    pub fn new(id: String) -> Self {
        Identifier {
            id,
            type_of: "Dynamic".to_string(),
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

#[derive(Debug, Clone, Default)]
pub struct EstaStruct {
    pub id: String,
    pub tag: usize,                     // Unique identifier
    pub size: usize,                    // Size of the entire struct
    pub fields: HashMap<String, usize>, // Offset of each field.
}

impl EstaStruct {
    pub fn new(s: Stmt) -> EstaStruct {
        if let Stmt::Struct(id, fields_list) = s {
            let tag = 0;
            let size = 2 + fields_list.len();
            //            let fields = HashMap::new();
            let mut fields = HashMap::new();
            for (i, field) in fields_list.iter().enumerate() {
                fields.insert(field.id.clone(), i + 2);
            }
            EstaStruct {
                id,
                tag,
                size,
                fields,
            }
        } else {
            Default::default()
        }
    }
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

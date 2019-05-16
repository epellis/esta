use crate::vm::bytecode::*;
use crate::vm::EstaData;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Program {
    pub insts: Vec<u8>,
    pub consts: HashMap<String, Vec<EstaData>>,
    // TODO: Eventually this can be merged into the function call opcode as a parameter
    pub context_alloc: HashMap<String, usize>,
    ctx: AsmCtx,
}

/// Assembly Context
///
/// This struct is used as the intermediate representation of a program before
/// all parts are compiled together
#[derive(Debug, Default, Clone)]
pub struct AsmCtx {
    pub base: String,
    pub blocks: Vec<MetaInst>,
    pub suffix: usize,
    //    context_alloc: HashMap<String, usize>,
}

impl AsmCtx {
    pub fn new_metainst(blocks: Vec<MetaInst>) -> AsmCtx {
        AsmCtx {
            blocks,
            ..Default::default()
        }
    }

    pub fn assemble(self) -> Program {
        let insts = self
            .blocks
            .iter()
            .map(|i| match i {
                MetaInst::ByteCode(b) => vec![b.clone().into()],
                MetaInst::Number(n) => n.to_le_bytes().to_vec(),
                MetaInst::Label(_) => Vec::new(),
                MetaInst::Const(_) => Vec::new(),
            })
            .flatten()
            .collect();

        Program {
            insts,
            ..Default::default()
        }
    }
}

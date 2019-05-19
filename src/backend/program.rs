use crate::vm::bytecode::*;
use crate::vm::EstaData;
use itertools::Itertools;
use std::collections::HashMap;

/// Program
///
/// This struct contains all information necessary to run an Esta program
/// TODO: Integrate with Serde so programs can be stored and loaded
#[derive(Debug, Default)]
pub struct Program {
    pub insts: Vec<u8>,
    pub consts: Vec<EstaData>,
}

/// Assembly Context
///
/// This struct is used as the intermediate representation of a program before
/// all parts are compiled together. Once it is complete, the assemble() method
/// is called, which outputs a new program struct
#[derive(Debug, Default, Clone)]
pub struct AsmCtx {
    pub base: String,
    pub blocks: Vec<MetaInst>,
    pub suffix: usize,
    pub declarations: Vec<String>, // Vec of local variables names declared in scope
}

impl AsmCtx {
    pub fn new_metainst(blocks: Vec<MetaInst>) -> AsmCtx {
        AsmCtx {
            blocks,
            ..Default::default()
        }
    }

    pub fn assemble(self) -> Program {
        let (consts, consts_map) = AsmCtx::make_consts(&self.blocks);

        let blocks = AsmCtx::resolve_identifiers(self.blocks);

        let insts = blocks
            .iter()
            .map(|i| match i {
                MetaInst::ByteCode(b) => vec![b.clone().into()],
                MetaInst::Number(n) => n.to_le_bytes().to_vec(),
                MetaInst::Label(_) => panic!("Label found in processed bytecode"),
                MetaInst::Const(c) => {
                    let offset = consts_map.get(c).unwrap().clone() as i16;
                    offset.to_le_bytes().to_vec()
                }
                MetaInst::Identifier(_) => panic!("Identifier found in processed bytecode"),
                MetaInst::Declaration(_) => panic!("Declaration found in processed bytecode"),
            })
            .flatten()
            .collect();

        Program { insts, consts }
    }

    // Scan the bytecode and create a consts section from every const bytecode
    fn make_consts(blocks: &Vec<MetaInst>) -> (Vec<EstaData>, HashMap<EstaData, usize>) {
        // TODO: Sort and dedup to remove unnecessary duplicates
        let consts = blocks
            .iter()
            .filter_map(|i| match i {
                MetaInst::Const(d) => Some(d),
                _ => None,
            })
            .cloned()
            .collect::<Vec<EstaData>>();

        let consts_map: HashMap<EstaData, usize> = consts
            .iter()
            .enumerate()
            .map(|(i, d)| (d.clone(), i))
            .collect();
        (consts, consts_map)
    }

    // For each identifier, map the declaration's stack offset and declaration offset
    // This in effect transforms bytecode like:
    // > LOADV foo
    // to
    // > LOADV 2 3
    // where foo is the third local variable declared two stacks away
    fn resolve_identifiers(blocks: Vec<MetaInst>) -> Vec<MetaInst> {
        // Replace all identifiers with their offsets
        let blocks: Vec<MetaInst> = blocks
            .iter()
            .enumerate()
            .map(|(idx, inst)| -> Vec<MetaInst> {
                match inst {
                    MetaInst::Identifier(id) => AsmCtx::find_declaration(&blocks, idx, id),
                    inst => vec![inst.clone()],
                }
            })
            .flatten()
            .collect();

        // Now get rid of all declarations because they are no longer necessary
        let blocks: Vec<MetaInst> = blocks
            .into_iter()
            .filter_map(|inst| match inst {
                MetaInst::Declaration(_) => None,
                inst => Some(inst),
            })
            .collect();

        blocks
    }

    // This helper method looks for an id's declaration in the most recent stack.
    fn find_declaration(blocks: &Vec<MetaInst>, idx: usize, id: &str) -> Vec<MetaInst> {
        println!("Searching for {}", id);
        let mut stack_offset = 0;
        let mut decl_offset = 0;
        for mut idx in (0..idx).rev() {
            let inst = &blocks[idx];
            match inst {
                MetaInst::Declaration(decl) if decl == id => {
                    // Count how many declarations come before this one
                    idx -= 1;
                    while let MetaInst::Declaration(_) = &blocks[idx] {
                        idx -= 1;
                        decl_offset += 1;
                    }

                    return vec![
                        MetaInst::Number(stack_offset),
                        MetaInst::Number(decl_offset),
                    ];
                }
                MetaInst::ByteCode(bc) if bc == &ByteCode::POPE => {
                    stack_offset -= 1;
                }
                MetaInst::ByteCode(bc) if bc == &ByteCode::PUSHE => {
                    stack_offset += 1;
                }
                _ => {}
            }
        }

        error!("Couldn't find {}", id);
        panic!("Declaration not found!");
    }

    pub fn next_label(&mut self) -> String {
        let suffix = self.suffix;
        self.suffix += 1;
        format!("{}-{}", self.base, suffix)
    }
}

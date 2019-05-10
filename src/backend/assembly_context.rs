use super::allocation::Alloc;
use crate::frontend::ast::EstaStruct;
use crate::middleend::MetaData;
use crate::util::stack::Stack;
use crate::util::string_hash32;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AsmCtx {
    base: String,                          // Tracks which function is being built
    scopes: HashMap<String, Stack<Alloc>>, // Tracks all scoped variables and their locations
    pub locals: HashMap<String, usize>,    // Tracks all local variables allocated within a function
    pub args: usize,                       // Count number of args for current function
    suffix: usize,                         // Keeps a unique suffix for each label
    md: MetaData,                          // Keep track of declared functions and structs
}

impl AsmCtx {
    pub fn new(md: MetaData) -> AsmCtx {
        AsmCtx {
            base: "GLOBAL".to_string(),
            scopes: HashMap::new(),
            locals: HashMap::new(),
            suffix: 0,
            args: 0,
            md,
        }
    }
    pub fn next_label(&mut self) -> String {
        self.suffix += 1;
        format!("{}_{}", self.base, self.suffix)
    }
    pub fn push_scope(&mut self) {
        self.scopes.get_mut(&self.base).unwrap().push(Alloc::new());
    }
    pub fn pop_scope(&mut self) {
        self.scopes.get_mut(&self.base).unwrap().pop();
    }
    pub fn add_fun(&mut self, id: &str) {
        self.base = id.to_string();
        self.scopes.insert(self.base.clone(), Stack::new());
        self.locals.insert(self.base.clone(), 0);
        self.push_scope();
    }
    pub fn pop_fun(&mut self) {
        self.pop_scope();
        self.base = "GLOBAL".to_string();
    }
    pub fn define(&mut self, id: &str) {
        *self.locals.get_mut(&self.base).unwrap() += 1;
        self.scopes
            .get_mut(&self.base)
            .and_then(|s| s.top())
            .unwrap()
            .define(id);
    }
    pub fn define_arg(&mut self, id: &str) {
        self.scopes
            .get_mut(&self.base)
            .and_then(|s| s.top())
            .unwrap()
            .define_arg(id);
    }
    pub fn get(&self, id: &str) -> Result<i64, &'static str> {
        let stack = self.scopes.get(&self.base).unwrap();
        for scope in stack.iter() {
            if scope.get(id).is_some() {
                return Ok(scope.get(id).unwrap());
            }
        }
        Err("Couldn't find id")
    }
    pub fn get_esta_struct(&self, id: &str) -> Option<EstaStruct> {
        for s in &self.md.structs {
            if s.id == id {
                return Some(s.clone());
            }
        }
        None
    }
    pub fn assemble_data_segment(&self) -> Vec<u64> {
        // Construct a vec of mappings from the hash of a field to it's offset
        // in the struct
        let struct_seg = self
            .md
            .structs
            .iter()
            .map(|s| {
                s.fields
                    .keys()
                    .cloned()
                    .map(string_hash32)
                    .interleave_shortest(0..)
                    .collect::<Vec<u64>>()
            })
            .collect::<Vec<Vec<u64>>>();

        // Construct a vec of offsets, where every offset is the offset of the struct
        // at that index
        let offsets = struct_seg
            .iter()
            .scan(0, |off, v| {
                let old_off = *off;
                *off += v.len();
                Some(old_off)
            })
            .map(|p| p + struct_seg.len())
            .map(|p| p as u64)
            .collect::<Vec<u64>>();

        offsets
            .into_iter()
            .chain(struct_seg.into_iter().flatten())
            .collect::<Vec<u64>>()
    }
}

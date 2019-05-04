use crate::frontend::ast::*;
use crate::vm::bytecode::*;
use regex::Regex;

/// Takes a string of text and deserialize it into valid (hopefully) bytecode
/// or vice versa
pub struct Converter {}

impl Converter {
    pub fn raw_to_inst(raw: &str) -> Result<Vec<Inst>, &'static str> {
        let mut inst = Vec::new();
        for cap in RE_INST.captures_iter(raw) {
            let bc = cap.get(1).and_then(|bc| RAW_TO_BYTE.get(bc.as_str()));
            let data = cap
                .get(2)
                .and_then(|data| data.as_str().trim().parse::<i64>().ok());
            match (bc, data) {
                (Some(b), Some(d)) => inst.push(Inst::new_data(b.clone(), d)),
                (Some(b), None) => inst.push(Inst::new_inst(b.clone())),
                _ => {}
            }
        }

        Ok(inst)
    }
}

lazy_static! {
    static ref RE_INST: Regex = Regex::new(r"([[:alpha:]]+)( (-)?[\d]+)?").unwrap();
}

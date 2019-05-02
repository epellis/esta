use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Alloc {
    scope: HashMap<String, usize>,
}

impl Alloc {
    pub fn new() -> Self {
        Alloc {
            scope: HashMap::new(),
        }
    }
    pub fn define(&mut self, id: &str) {}
    pub fn get(&self, id: &str) -> Option<usize> {
        self.get(id)
    }
}

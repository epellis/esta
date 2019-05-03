use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Alloc {
    scope: HashMap<String, usize>,
    top: usize,
}

impl Alloc {
    pub fn new() -> Self {
        Alloc {
            scope: HashMap::new(),
            top: 0,
        }
    }
    pub fn define(&mut self, id: &str) {
        self.scope.insert(id.to_string(), self.top);
        self.top += 1;
    }
    pub fn get(&self, id: &str) -> Option<usize> {
        self.scope.get(id).cloned()
    }
}

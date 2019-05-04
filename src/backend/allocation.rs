use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Alloc {
    scope: HashMap<String, i64>,
    top: i64,
    bot: i64,
}

impl Alloc {
    pub fn new() -> Self {
        Alloc {
            scope: HashMap::new(),
            top: 0,
            bot: -3,
        }
    }
    pub fn define(&mut self, id: &str) {
        self.scope.insert(id.to_string(), self.top);
        self.top += 1;
    }
    pub fn define_arg(&mut self, id: &str) {
        self.scope.insert(id.to_string(), self.bot);
        self.bot -= 1;
    }
    pub fn get(&self, id: &str) -> Option<i64> {
        self.scope.get(id).cloned()
    }
}

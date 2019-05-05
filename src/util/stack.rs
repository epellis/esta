#[derive(Debug, Clone, Default)]
pub struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    pub fn top(&self) -> Option<&T> {
        self.stack.last()
    }

    pub fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    pub fn push(&mut self, val: T) {
        self.stack.push(val);
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            stack: &self.stack,
            pos: self.stack.len(),
        }
    }
}

pub struct Iter<'a, T> {
    stack: &'a Vec<T>,
    pos: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos > 0 {
            self.pos -= 1;
            Some(&self.stack[self.pos])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::Stack;

    #[test]
    fn core() {
        let mut stack: Stack<usize> = Stack::new();
        stack.push(0);
        stack.push(1);
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), Some(0));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn iter() {
        let mut stack: Stack<usize> = Stack::new();
        stack.push(0);
        stack.push(1);

        let mut iter = stack.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&0));
        assert_eq!(iter.next(), None);
    }
}

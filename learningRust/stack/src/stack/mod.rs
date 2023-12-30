use std::clone;
use std::ops::{Add, Mul};

#[derive(Debug)]
pub struct Stack<T> {
    data: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { data: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.data.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }
}

impl<T> StackOps<T> for Stack<T>
where
    T: Default + clone::Clone + Add<Output = T> + Mul<Output = T>,
{
    fn sum(&self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(
                self.data
                    .iter()
                    .cloned()
                    .fold(Default::default(), |acc, x| acc + x),
            )
        }
    }

    fn prod(&self, init: T) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.data.iter().cloned().fold(init, |acc, x| acc * x))
        }
    }
}

pub trait StackOps<T> {
    fn sum(&self) -> Option<T>;
    fn prod(&self, init: T) -> Option<T>;
}

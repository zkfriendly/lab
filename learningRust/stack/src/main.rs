use stack::Stack;

use crate::stack::StackOps;

mod stack;

fn main() {
    let mut stack: Stack<u32> = Stack::new();

    stack.push(1);
    stack.push(2);
    stack.push(4);

    let sum = stack.sum();
    let prod = stack.prod(1);

    println!("sum: {:?}, prod: {:?}", sum, prod);
}

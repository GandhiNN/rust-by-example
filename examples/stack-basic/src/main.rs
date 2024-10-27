struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }
    // A vector may be empty so we return an Option type
    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
    fn push(&mut self, item: T) {
        self.stack.push(item)
    }
    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
    fn length(&self) -> usize {
        self.stack.len()
    }
    // A vector may be empty so we return an Option type.
    // But we don't want to return the owned type as it's equal to
    // removing the element itself.
    // Hence we are returning only the reference to that element.
    fn peek(&self) -> Option<&T> {
        self.stack.last()
    }
}

fn main() {
    // Let's start with an empty stack
    let mut empty_stack: Stack<isize> = Stack::new();
    // Push and pop an element
    empty_stack.push(1);
    let item = empty_stack.pop();
    assert_eq!(item.unwrap(), 1);
    // Check if it's empty
    println!("Is stack empty: {}", empty_stack.is_empty());

    // Let's try using a Stack with a vector attribute having some elements inside.
    let filled_stack: Stack<isize> = Stack {
        stack: vec![0, 1, 2, 3, 4, 5],
    };
    // Get the last element
    let last = filled_stack.peek().unwrap();
    println!("Last element is: {}", last);
    // Check if we still have all the elements
    println!("Stack: {:?}", filled_stack.stack);
    let stack_length = filled_stack.length();
    println!("Stack length: {}", stack_length);
    // Check if it's empty
    println!("Is stack empty: {}", filled_stack.is_empty());
}

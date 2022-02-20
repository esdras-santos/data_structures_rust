
pub struct Stack<T>{
    stack: Vec<T>
}

impl<T> Stack<T> {
    pub fn new() -> Self{
        Stack{ stack: Vec::new()}
    }

    pub fn size(&self) -> usize{
        self.stack.len()
    }

    pub fn is_empty(&self) -> bool{
        self.size() == 0
    }

    pub fn push(&mut self, elem: T){
        self.stack.push(elem)
    }

    pub fn pop(&mut self) -> T{
        self.stack.pop().unwrap()
    }

    pub fn peek(&self) -> &T{
        self.stack.last().unwrap()
    }
}
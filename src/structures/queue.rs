
pub struct Queue<T>{
    queue: Vec<T>
}

impl<T> Queue<T> {
    pub fn new() -> Self{
        Queue{ queue: Vec::new()}
    }

    pub fn size(&self) -> usize{
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool{
        self.size() == 0
    }

    pub fn peek(&self) -> &T{
        self.queue.first().unwrap()
    }

    pub fn poll(&mut self) -> T{
        self.queue.remove(0)
    }

    pub fn offer(&mut self, elem: T){
        self.queue.push(elem)
    }


}
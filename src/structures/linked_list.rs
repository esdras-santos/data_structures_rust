
pub struct List{
    
    pub head: Option<Box<Node>>,
    pub tail: Option<Box<Node>>,
    pub size: u64,
}
#[derive(Clone)]
struct Node{
    pub prev: Option<Box<Node>>,
    pub next: Option<Box<Node>>,
    pub data: u64,
}

impl Node{
    pub fn new(data: u64, next: Option<Box<Node>>, prev: Option<Box<Node>>) -> Self {
        Node {prev, next, data}
    }
}

impl List {
    pub fn new() -> Self{
        List { head: None, tail: None, size: 0 }
    }

    pub fn clear(mut self){
        self.head = None;
        self.tail = None;
        self.size = 0;
    }

    pub fn size(&self) -> u64{
        self.size
    }

    pub fn is_empty(&self) -> bool{
        self.size() == 0
    } 

    pub fn add(&self, data: u64){

    }

    pub fn add_first(&mut self, data: u64){
        if self.is_empty(){
            self.head = Some(Box::new(Node{prev: None, next: None, data}));
            self.tail = Some(Box::new(Node{prev: None, next: None, data}));
        } else{
            let aux = self.head.as_ref().unwrap().clone();
            self.head.unwrap().prev = Some(Box::new(Node::new(data,Some(self.head.as_ref().unwrap().clone()),None)));
            self.head = Some(Box::new(Node::new(data,Some(aux),None)));
        }
        self.size += 1;
    }

    pub fn add_last(&self, data: u64){
        
    }
}




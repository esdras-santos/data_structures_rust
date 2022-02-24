
pub struct BinarySearchTree<T>{
    node_count: u64,
    root: Option<Node<T>>,
}

struct Node<T>{
    data: Option<T>,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    pub fn new(left: Option<Box<Node<T>>>, right: Option<Box<Node<T>>>, elem: Option<T>) -> Self{
        Self{
            left,
            right,
            data: elem
        }
    }
}

impl<T> BinarySearchTree<T> {
    pub fn is_empty(&self) -> bool{
        self.size() == 0
    }

    pub fn size(&self) -> u64{
        self.node_count
    }

    pub fn add(&mut self, elem: T) -> bool{
        if self.contains(&elem) {
            false
        } else {
            self.root = self.add_node(self.root ,elem);
            self.node_count += 1;
            true
        }
    }

    fn add_node(&self, mut node: Option<Node<T>>, elem: T) -> Node<T>{
        if node.is_none(){
            let aux = Node::new(None,None,elem);
            node = Some(aux);
        } else {
            if elem < node.unwrap().data.unwrap(){
                let mut parent = *node.unwrap().left.unwrap();
                node.unwrap().left = Some(Box::new(self.add_node(Some(parent), elem)))
            } else {
                let mut parent = *node.unwrap().right.unwrap();
                node.unwrap().right = Some(Box::new(self.add_node(Some(parent), elem)))
            }
        }

        node
    }

    pub fn remove(&mut self, elem: T) -> bool{
        if self.contains(&elem){
            self.root = self.remove_node(self.root, elem);
            self.node_count -= 1;
            true
        }
        false
    }
    
    fn remove_node(&mut self, node: Option<Node<T>>, elem: T) -> Option<Node<T>>{
        if node.is_none(){
            None
        }

        if elem < node.unwrap().data.unwrap(){
            let aux = *node.unwrap().left.unwrap();
            node.unwrap().left = Some(Box::new(self.remove_node(Some(aux), elem)));
        } else if elem > node.unwrap().data.unwrap(){
            let aux = *node.unwrap().right.unwrap();
            node.unwrap().right = Some(Box::new(self.remove_node(Some(aux), elem)));
        } else {
            if node.unwrap().left.is_none(){
                let right = *node.unwrap().right.unwrap();
                node.unwrap().data = None;
                node = None;
                return Some(right);
            } else if node.unwrap().right.is_none(){
                let left = *node.unwrap().left.unwrap();
                node.unwrap().data = None;
                node = None;
                return Some(left);
            } else {
                let tmp = self.dig_left(*node.unwrap().right.unwrap());
                let aux = *node.unwrap().right.unwrap();
                node.unwrap().data = tmp.unwrap().data;
                node.unwrap().right = self.remove_node(Some(aux), tmp.unwrap().data);

                // let tmp = self.dig_right(*node.unwrap().left.unwrap());
                // let aux = *node.unwrap().left.unwrap();
                // node.unwrap().data = tmp.unwrap().data;
                // node.unwrap().left = self.remove_node(Some(aux), tmp.unwrap().data);

            }
        }

        node
    }

    fn dig_left(&self, node: Node<T>) -> Node<T>{
        let mut cur = node;
        while !cur.left.is_none(){
            cur = *cur.left.unwrap()
        }
        cur
    }

    fn dig_right(&self, node: Node<T>) -> Node<T>{
        let mut cur = node;
        while !cur.right.is_none(){
            cur = *cur.right.unwrap()
        }
        cur
    }

    pub fn contains(&self, elem: T) -> bool{
        return self.contains_node(self.root, elem);
    }

    fn contains_node(&self, node: Option<Node<T>>, elem: T) -> bool{
        if node.is_none(){
            false
        }

        if elem < node.unwrap().data.unwrap(){
            let aux = *node.unwrap().left.unwrap();
            return self.contains_node(aux.left, elem);
        } else if elem > node.unwrap().data.unwrap(){
            let aux = *node.unwrap().right.unwrap();
            return self.contains_node(aux.right, elem);
        } else{
            return true;
        }
    }

    pub fn height(&self) -> u64{
        return height_tree(self.root);
    }

    fn height_tree(&self, node: Option<Node<T>>) -> u64{
        if node.is_none(){
            return 0;
        }
        let auxleft = *node.unwrap().left.unwrap();
        let auxright = *node.unwrap().right.unwrap();
        return (self.height_tree(Some(auxleft)) + self.height_tree(Some(auxright))) + 1;
    }
}
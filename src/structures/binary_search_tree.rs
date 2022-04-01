#[derive(Debug, Clone)]
pub struct BinarySearchTree<T> where T: std::cmp::Eq + std::hash::Hash + std::cmp::PartialOrd + std::marker::Copy{
    node_count: u64,
    root: Option<Node<T>>,
}

#[derive(Debug, Clone)]
struct Node<T> where T: std::cmp::Eq + std::hash::Hash + std::cmp::PartialOrd + std::marker::Copy{
    data: Option<T>,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>
}

impl<T> Node<T> where T: std::cmp::Eq + std::hash::Hash + std::cmp::PartialOrd + std::marker::Copy{
    pub fn new(left: Option<Box<Node<T>>>, right: Option<Box<Node<T>>>, elem: Option<T>) -> Self{
        Self{
            left,
            right,
            data: elem
        }
    }
}

impl<T> BinarySearchTree<T> where T: std::cmp::Eq + std::hash::Hash + std::cmp::PartialOrd + std::marker::Copy {
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

    fn add_node(&self, mut node: Option<Node<T>>, elem: T) -> Option<Node<T>>{
        if node.is_none(){
            let aux = Node::new(None,None,Some(elem));
            node = Some(aux);
        } else {
            if elem < node.unwrap().data.unwrap(){
                let mut parent = *node.unwrap().left.unwrap();
                node.unwrap().left = Some(Box::new(self.add_node(Some(parent), elem).unwrap()))
            } else {
                let mut parent = *node.unwrap().right.unwrap();
                node.unwrap().right = Some(Box::new(self.add_node(Some(parent), elem).unwrap()))
            }
        }

        node
    }

    pub fn remove(&mut self, elem: T) -> bool{
        if self.contains(&elem){
            let aux = self.root.unwrap();
            self.root = self.remove_node(&Some(aux), elem);
            self.node_count -= 1;
            return true;
        }
        false
    }
    
    fn remove_node(&mut self, mut node: &Option<Node<T>>, elem: T) -> Option<Node<T>>{
        if node.is_none(){
            return None;
        }

        if elem < node.unwrap().data.unwrap(){
            let aux = *node.unwrap().left.unwrap();
            node.unwrap().left = Some(Box::new(self.remove_node(&Some(aux), elem).unwrap()));
        } else if elem > node.unwrap().data.unwrap(){
            let aux = *node.unwrap().right.unwrap();
            node.unwrap().right = Some(Box::new(self.remove_node(&Some(aux), elem).unwrap()));
        } else {
            if node.unwrap().left.is_none(){
                let right = *node.unwrap().right.unwrap();
                node.unwrap().data = None;
                node = &None;
                return Some(right);
            } else if node.unwrap().right.is_none(){
                let left = *node.unwrap().left.unwrap();
                node.unwrap().data = None;
                node = &None;
                return Some(left);
            } else {
                let tmp = self.dig_left(*node.unwrap().right.unwrap());
                let aux = *node.unwrap().right.unwrap();
                node.unwrap().data = tmp.data;
                node.unwrap().right = Some(Box::new(self.remove_node(&Some(aux), tmp.data.unwrap()).unwrap()));

                // let tmp = self.dig_right(*node.unwrap().left.unwrap());
                // let aux = *node.unwrap().left.unwrap();
                // node.unwrap().data = tmp.unwrap().data;
                // node.unwrap().left = self.remove_node(Some(aux), tmp.unwrap().data);

            }
        }
        
        return node.clone();
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

    pub fn contains(&self, elem: &T) -> bool{
        let aux = self.root.as_ref().unwrap();
        return self.contains_node(Some(aux), *elem);
    }

    fn contains_node(&self, node: Option<&Node<T>>, elem: T) -> bool{
        if node.is_none(){
            return false;
        }

        if elem < node.unwrap().data.unwrap(){
            let aux = &*node.unwrap().left.as_ref().unwrap();
            return self.contains_node(Some(aux), elem);
        } else if elem > node.unwrap().data.unwrap(){
            let aux = &*node.unwrap().right.as_ref().unwrap();
            return self.contains_node(Some(aux), elem);
        } else{
            return true;
        }
    }

    pub fn height(&self) -> u64{
        
        return self.height_tree(self.root.as_ref().unwrap());
    }

    fn height_tree(&self, node: &Node<T>) -> u64{
        if node.data.is_none(){
            return 0;
        }
        let auxleft = &**node.left.as_ref().unwrap();
        let auxright = &**node.right.as_ref().unwrap();
        return (self.height_tree(auxleft) + self.height_tree(auxright)) + 1;
    }
}
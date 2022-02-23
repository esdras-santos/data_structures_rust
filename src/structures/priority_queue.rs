use std::collections::HashMap;


pub struct PQueue<T> where T: std::cmp::Eq + std::hash::Hash + std::cmp::PartialOrd + std::marker::Copy{
    heap: Vec<T>,
    heap_size: u64,
    capacity: u64,
    track: HashMap<T,Vec<usize>>

}

impl<T> PQueue<T> where T: std::cmp::Eq + std::hash::Hash + std::cmp::PartialOrd + std::marker::Copy {
    pub fn new(elems: Option<Vec<T>>) -> Self{
        if elems.is_none(){
            PQueue{heap: Vec::new(), heap_size:0, capacity: 0, track: HashMap::new()}    
        }else{
            let heap: Vec<T> = Vec::with_capacity(elems.as_ref().unwrap().len());
            let mut pq: PQueue<T> = PQueue{heap: Vec::new(), heap_size:elems.as_ref().unwrap().len() as u64, capacity: elems.as_ref().unwrap().len() as u64,track: HashMap::new()};
            for i in 0..pq.heap_size{
                pq.add_track(elems.as_ref().unwrap()[i as usize].clone(), i as usize);
                pq.heap.push(elems.as_ref().unwrap()[i as usize].clone());
            }
            let mut j: u64 = (pq.heap_size/2)-1;
            while  j >= 0{
                pq.sink(j);
                j-=1;
            }  
            pq
        }
        
    }

    pub fn is_empty(&self) -> bool{
        self.heap_size == 0
    }

    pub fn clear(&mut self){
        self.heap.clear();
        self.heap_size = 0;
        self.track.clear()
    }

    pub fn size(&self) -> u64{
        self.heap_size
    }

    pub fn peek(&self) -> &T{
        self.heap.first().unwrap()
    }

    pub fn poll(&mut self) -> T{
        self.remove_at(0).unwrap()
    }

    pub fn contains(&self, elem: T) -> bool{
        self.track.contains_key(&elem)
    }

    pub fn add(&mut self, elem: T){
        if self.heap_size < self.capacity{
            self.heap[self.heap_size as usize] = elem
        } else {
            self.heap.push(elem);
            self.capacity += 1
        }

        self.add_track(elem, self.heap_size as usize);
        self.swin(self.heap_size);
        self.heap_size+=1;
    }

    fn less(&self, i: u64, j: u64) -> bool{
        let node1 = self.heap[i as usize];
        let node2 = self.heap[j as usize];

        node1 <= node2
    }

    fn swin(&mut self,mut k: u64) {
        let mut parent = (k-1) / 2;
        while k > 0 && self.less(k, parent){
            self.swap(parent , k);
            k = parent;
            parent = (k-1) / 2
        }

    }

    fn sink(&mut self, mut k: u64){
        while true{
            let left = 2*k+1;
            let right = 2*k+2;
            let mut smallest = left;

            if right < self.heap_size && self.less(right, left){
                smallest = left
            }

            if left >= self.heap_size || self.less(k, smallest){
                break;
            }

            self.swap(smallest, k);
            k = smallest
        }
    }

    fn swap(&mut self, i: u64, j: u64){
        let i_elem = self.heap[i as usize];
        let j_elem = self.heap[j as usize];

        self.heap[i as usize] = j_elem;
        self.heap[j as usize] = i_elem;

        self.swap_track(i_elem, j_elem, i as usize, j as usize);
    }

    pub fn remove(&mut self, elem: T) -> bool{
        let index = self.get_track(elem);

        if !index.is_none() {
            self.remove_at(index.unwrap());
        }

        !index.is_none()
    }

    fn remove_at(&mut self, i: usize) -> Option<T>{
        if self.is_empty() {
            return None
        }

        self.heap_size -= 1;
        let removed_data = self.heap[i];
        self.swap(i.try_into().unwrap(), self.heap_size);

        self.heap.remove(self.heap_size as usize);
        self.remove_track(&removed_data, self.heap_size as usize);

        if i as u64 == self.heap_size {
            return Some(removed_data)
        }

        let elem = self.heap[i];
        self.sink(i.try_into().unwrap());
        if self.heap[i] == elem {
            self.swin(i.try_into().unwrap())
        }

        Some(removed_data)
    }

    fn add_track(&mut self, value: T, index: usize){
        let mut set = self.track.get(&value).unwrap().clone().to_vec();

        if set.is_empty(){
            let mut ni = Vec::new();
            ni.push(index);
            self.track.insert(value, ni);
        }else{
            set.push(index);
            self.track.insert(value, set);
        }
    }

    fn remove_track(&mut self, value: &T, index: usize){
        let mut set = self.track.get(&value).unwrap().clone().to_vec();
        set.remove(index);
        if set.is_empty(){
            self.track.remove(&value);
        } else{
            self.track.insert(*value, set);
        }
    }

    fn get_track(&self, value: T) -> Option<usize>{
        let set = self.track.get(&value);
        if !set.is_none(){
            return Some(*set.unwrap().last().unwrap())
        }
        return None
    }

    fn swap_track(&mut self, val1: T, val2: T, val1Index: usize, val2Index: usize) {
        let mut set1 = self.track.get(&val1).unwrap().clone().to_vec();
        let mut set2 = self.track.get(&val2).unwrap().clone().to_vec();

        set1.remove(val1Index);
        set2.remove(val2Index);

        set1.push(val2Index);
        set2.push(val1Index);

        self.track.insert(val1, set1.clone().to_vec());
        self.track.insert(val2, set2.clone().to_vec());
    }
}



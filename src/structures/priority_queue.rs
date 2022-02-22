use std::collections::HashMap;


pub struct PQueue<T>{
    heap: Vec<T>,
    heap_size: u64,
    capacity: u64,
    track: HashMap<T,Vec<T>>

}

impl<T> PQueue<T> {
    pub fn new(elems: Option<Vec<T>>) -> Self{
        if elems.is_none(){
            PQueue{heap: Vec::new(), heap_size:0, capacity: 0, track: HashMap::new()}    
        }else{
            let heap = Vec::with_capacity(elems.unwrap().len());
            let pq: PQueue<T> = PQueue{heap: Vec::new(), heap_size:elems.unwrap().len(), capacity: elems.unwrap().len(),track: HashMap::new()};
            for i in 0..pq.heap_size{
                pq.add_track(elems.unwrap()[i], i);
                pq.heap.push(elems[i]);
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
        self.heap == 0
    }

    pub fn clear(&mut self){
        self.heap.clear();
        self.heap_size = 0;
        self.track.clear()
    }

    pub fn size(&self) -> i64{
        self.heap_size
    }

    pub fn peek(&self) -> &T{
        self.heap.first().unwrap()
    }

    pub fn poll(&mut self) -> T{
        self.remove_at(0)
    }

    pub fn contains(&self, elem: T) -> bool{
        self.track.contains_key(&elem)
    }

    pub fn add(&mut self, elem: T){
        if self.heap_size < self.capacity{
            self.heap[self.heap_size] = elem
        } else {
            self.heap.push(elem);
            self.capacity += 1
        }

        self.add_track(elem, self.heap_size);
        self.swim(self.heap_size);
        self.heap_size+=1;
    }

    fn less(&self, i: u64, j: u64) -> bool{
        let node1 = self.heap[i as usize];
        let node2 = self.heap[j as usize];

        node1 <= node2
    }

    fn swin(&self, k: u64) {
        let mut parent = (k-1) / 2;
        while k > 0 && less(k, parent){
            self.swap(parent , k);
            k = parent;
            parent = (k-1) / 2
        }

    }

    fn sink(&self, mut k: u64){
        while(true){
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

        self.map_swap(i_elem, j_elem, i, j) 
    }

    pub fn remove(&self, elem: T) -> bool{
        let index = self.get_track(elem);

        if index != None {
            self.removeAt(index.unwrap())
        }

        index != None
    }

    fn removeAt(&mut self, i: usize) -> Option<T>{
        if self.is_empty() {
            None
        }

        self.heap_size -= 1;
        let removed_data = self.heap[i];
        self.swap(i, self.heap_size);

        self.heap.remove(self.heap_size as usize);
        self.remove_track(&removed_data, self.heap_size);

        if i == self.heap_size {
            return Some(removed_data)
        }

        let elem = self.heap[i];
        self.sink(i);
        if self.heap[i] == elem {
            self.swin(i)
        }

        Some(removed_data)
    }

    fn add_track(&mut self, value: T, index: usize){
        let mut set = self.track.get(&value);

        if set.is_none(){
            let ni = Vec::new();
            ni.push(index);
            self.track.insert(value, ni)
        }else{
            set.unwrap().push(index);
            self.track.insert(value, *set.unwrap())
        }
    }

    fn remove_track(&mut self, value: T, index: usize){
        let set = self.track.get(&value);
        set.unwrap().remove(index);
        if set.unwrap.len() == 0{
            self.track.remove(&value)
        } else{
            self.track.insert(&value, *set.unwrap())
        }
    }

    fn get_track(&self, value: T) -> Option<usize>{
        let set = self.track.get(&value);
        if !set.is_none(){
            return set.unwrap().last()
        }
        return None
    }

    fn swap_track(&self, val1: T, val2: T, val1Index: usize, val2Index: usize) {
        let set1 = self.track.get(&val1).unwrap();
        let set2 = self.track.get(&val2).unwrap();

        set1.remove(val1Index);
        set2.remove(val2Index);

        set1.push(val2Index);
        set2.push(val1Index);
    }
}



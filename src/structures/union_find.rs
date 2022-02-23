
pub struct UnionFind{
    size: u64,
    sz: Vec<u64>,
    id: Vec<u64>,
    num_components: u64
}

impl UnionFind {
    pub fn new(size: u64) -> Self{
        let mut sz: Vec<u64> = Vec::with_capacity(size as usize);
        let mut id: Vec<u64> = Vec::with_capacity(size as usize);
        for i in 0..size{
            id.push(i);
            sz.push(1)
        }
        Self{
            size,
            sz,
            id,
            num_components: size
        }
    }

    pub fn find(&mut self, mut p: u64) -> usize{
        let mut root = p;
        while root != self.id[root as usize] {
            root = self.id[root as usize]
        }

        // path compresion
        while p != root {
            let mut next = self.id[p as usize];
            self.id[p as usize] = root;
            p = next;
        }

        root as usize
    }

    pub fn connected(&mut self, p: u64, q: u64) -> bool{
        self.find(p) == self.find(q)
    }

    pub fn component_size(&mut self, p: u64) -> u64{
        let i = self.find(p);
        self.sz[i]
    }

    pub fn size(&self) -> u64{
        self.size
    }

    pub fn components(&self) -> u64{
        self.num_components
    }

    pub fn unify(&mut self, p: u64, q: u64){
        let root1 = self.find(p);
        let root2 = self.find(q);

        if root1 == root2{
            return
        }

        if self.sz[root1 as usize] < self.sz[root2 as usize] {
            self.sz[root2 as usize] += self.sz[root1 as usize];
            self.id[root1 as usize] = root2 as u64
        } else {
            self.sz[root1 as usize] += self.sz[root2 as usize];
            self.id[root2 as usize] = root1 as u64
        }

        self.num_components -= 1;
    }

    
}
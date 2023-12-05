use rand::{Rng, thread_rng};

struct RandomizedSet {
    array: Vec<i32>,
    map: std::collections::HashMap<i32, usize>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    fn new() -> Self {
        Self {
            array: Vec::new(),
            map: std::collections::HashMap::new(),
        }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        match self.map.get(&val) {
            None => {
                self.map.insert(val, self.array.len());
                self.array.push(val);
                true
            },
            Some(_) => false,
        }

    }
    
    fn remove(&mut self, val: i32) -> bool {
        match self.map.remove(&val) {
            Some(i) => {
                if i == self.array.len()-1 {
                    self.array.pop();
                } else {
                    self.array[i] = self.array.pop().unwrap();
                    self.map.insert(self.array[i], i);
                }
                true
            },
            None => false,
        }
    }
    
    fn get_random(&self) -> i32 {
        let x = rand::thread_rng().gen_range(0, self.array.len());
        self.array[x]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test() {
        // ["RandomizedSet","insert","insert","remove","insert","remove","getRandom"]
        // [[],[0],[1],[0],[2],[1],[]]
        let mut set = RandomizedSet::new();
        set.insert(0);
        set.insert(1);
        set.remove(0);
        set.insert(2);
        set.remove(1);

    }

}
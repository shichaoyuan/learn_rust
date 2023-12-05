
use rand::{Rng, thread_rng};

struct Solution {
    map: std::collections::HashMap<i32, i32>,
    sz: i32,
}


impl Solution {

    fn new(n: i32, blacklist: Vec<i32>) -> Self {
        let sz = n - blacklist.len() as i32;
        let mut map = std::collections::HashMap::new();
        for b in blacklist.iter() {
            map.insert(*b, 666);
        }

        let mut last = n - 1;
        for b in blacklist.iter() {
            if *b >= sz {
                continue;
            }
            while map.contains_key(&last) {
                last -= 1;
            }
            map.insert(*b, last);
            last -= 1;
        }



        Self {
            sz,
            map,
        }
    }
    
    fn pick(&self) -> i32 {
        let x = rand::thread_rng().gen_range(0, self.sz as i32);
        match self.map.get(&x) {
            Some(i) => *i,
            None => x,
        }
    }
}

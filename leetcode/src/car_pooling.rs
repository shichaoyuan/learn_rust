struct Difference {
    diff: Vec<i32>,
}

impl Difference {
    fn new(n: usize) -> Self {
        Difference { diff: vec![0; n] }
    }

    fn incr(&mut self, i: usize, j: usize, v: i32) {
        self.diff[i] += v;
        if j < self.diff.len() - 1 {
            self.diff[j + 1] -= v;
        }
    }

    fn result(mut self) -> Vec<i32> {
        for i in 1..self.diff.len() {
            self.diff[i] += self.diff[i - 1];
        }
        self.diff
    }
}

struct Solution;

impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut diff = Difference::new(1001);
        for trip in trips.into_iter() {
            diff.incr(trip[1] as usize, (trip[2] - 1) as usize, trip[0]);
        }

        let res = diff.result();
        for r in res.into_iter() {
            if r > capacity {
                return false;
            }
        }
        true
    }
}

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
    pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
        let mut diff = Difference::new(n as usize);
        for v in bookings.into_iter() {
            diff.incr((v[0] - 1) as usize, (v[1] - 1) as usize, v[2]);
        }
        diff.result()
    }
}

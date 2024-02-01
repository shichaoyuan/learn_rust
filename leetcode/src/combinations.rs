

struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut track: Vec<i32> = Vec::new();
        Self::backtrack(n as usize, k as usize, 1, &mut track, &mut res);
        res
    }

    fn backtrack(n: usize, k: usize, start: usize, track: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if track.len() == k {
            res.push(track.clone());
        } 
        for i in start..=n {
            track.push(i as i32);
            Self::backtrack(n, k, i+1, track, res);
            track.pop();
        }
    }
}
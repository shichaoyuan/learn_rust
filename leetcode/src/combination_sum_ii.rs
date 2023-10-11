
struct Solution;

struct Combinator {
    candicates: Vec<i32>,
    target: i32,
    sum: i32,
    track: Vec<i32>,
    result: Vec<Vec<i32>>
}

impl Combinator {
    pub fn combination_sum(mut self) -> Vec<Vec<i32>> {
        self.backtrack(0);
        self.result
    }

    pub fn backtrack(&mut self, start: usize) {
        if self.sum == self.target {
            self.result.push(self.track.clone());
            return;
        }

        if self.sum > self.target {
            return;
        }

        for i in start..self.candicates.len() {
            if i > start && self.candicates[i] == self.candicates[i-1] {
                continue;
            }
            self.sum += self.candicates[i];
            self.track.push(self.candicates[i]);
            self.backtrack(i+1);
            self.sum -= self.candicates[i];
            self.track.pop();

        }




    }
}

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort_unstable();
        let mut combinator = Combinator {
             candicates: candidates,
             target: target,
             sum: 0,
             track: Vec::new(),
             result: Vec::new()
        };

        combinator.combination_sum()
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::combination_sum2(
            vec![10,1,2,7,6,1,5], 8),
            vec![
                vec![1,1,6],
                vec![1,2,5],
                vec![1,7],
                vec![2,6]
            ]
        );
    }
}
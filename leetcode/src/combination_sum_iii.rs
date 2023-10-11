
struct Solution;

struct Combinator {
    candicates: Vec<i32>,
    target: i32,
    k: i32,
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
            if self.track.len() == self.k as usize {
                self.result.push(self.track.clone());
            }
            return;
        }

        if self.sum > self.target {
            return;
        }


        for i in start..self.candicates.len() {
            if self.track.len() > self.k as usize {
                return;
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
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut candidates = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut combinator = Combinator {
             candicates: candidates,
             target: n,
             k: k,
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
        assert_eq!(Solution::combination_sum3(3, 7),
            vec![
                vec![1,2,4],
            ]
        );
    }
}
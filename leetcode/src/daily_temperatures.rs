struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut result = vec![0_i32; n];
        let mut stack = std::collections::VecDeque::new();
        for i in (0..n).rev() {
            while let Some(&a) = stack.back() {
                if temperatures[a] <= temperatures[i] {
                    stack.pop_back();
                } else {
                    break;
                }
            }

            if let Some(&a) = stack.back() {
                result[i] = (a - i) as i32;
            }

            stack.push_back(i);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let res = Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]);
        assert_eq!(res, vec![1, 1, 4, 2, 1, 1, 0, 0]);
    }
}

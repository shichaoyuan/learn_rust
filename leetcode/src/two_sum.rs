
struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            if let Some(&v) = map.get(&(target-num)) {
                return vec![i as i32, v];
            }
            map.insert(num.clone(), i as i32);
        }
        panic!();
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
    
    #[test]
    fn test() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let r = Solution::two_sum(nums, target);
        assert_eq!(vec![1, 0], r);
    }
    
}
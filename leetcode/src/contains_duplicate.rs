struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = std::collections::HashSet::new();
        for num in nums {
            if set.contains(&num) {
                return true;
            }
            set.insert(num);
        }
        false
    }
}

mod test {
    use super::Solution;

    #[test]
    fn test() {
        let nums = vec![1, 2, 3, 1];
        let result = Solution::contains_duplicate(nums);
        assert_eq!(true, result)
    }

}
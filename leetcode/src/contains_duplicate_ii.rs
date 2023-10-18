struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut map = std::collections::HashMap::new();
        for (i, num) in nums.into_iter().enumerate() {
            if map.contains_key(&num) {
                let j = map.get(&num).unwrap();
                if i - *j <= k as usize {
                    return true;
                }
            }
            map.insert(num, i);
        }
        false
    }
}

mod test {
    use super::Solution;

    #[test]
    fn test() {
        let nums = vec![1, 2, 3, 1];
        let result = Solution::contains_nearby_duplicate(nums, 3);
        assert_eq!(true, result)
    }

}
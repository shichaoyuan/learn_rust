use std::future::IntoFuture;

struct Solution;

impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, index_diff: i32, value_diff: i32) -> bool {
        use std::ops::Bound::Unbounded;
        use std::ops::Bound::Included;
        let mut set = std::collections::BTreeSet::new();
        for i in 0..nums.len() {
            let num = nums[i];
            let lower = set.range((Unbounded, Included(num))).next_back();
            let upper = set.range((Included(num), Unbounded)).next();
            match lower {
                Some(a) => {
                    if num - *a <= value_diff {
                        return true;
                    } 
                },
                None => ()
            }
            match upper {
                Some(a) => {
                    if *a -num <= value_diff {
                        return true;
                    } 
                },
                None => ()
            }
            if i >= index_diff as usize {
                set.remove(&nums[i-(index_diff as usize)]);
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
        let result = Solution::contains_nearby_almost_duplicate(nums, 3, 0);
        assert_eq!(true, result)
    }

}
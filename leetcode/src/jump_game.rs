
struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let len = nums.len();
        let mut maxIndex: usize = 0;
        for (i, n) in nums.into_iter().enumerate() {
            if i <= maxIndex {
                maxIndex = maxIndex.max(i+(n as usize));
                if maxIndex >= (len-1) {
                    return true;
                }
            }
        }
        return false;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {

        let nums = vec![2,3,1,1,4];
        let r = Solution::can_jump(nums);
        assert_eq!(true, r);

    }
}
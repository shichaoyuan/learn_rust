
struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> i32 {

        let mut step = 0;
        let mut pos = nums.len() - 1;
        while pos > 0 {
            for (i, n) in nums.iter().enumerate() {
                if (i + (*n as usize)) >= pos {
                    step += 1;
                    pos = i;
                    break;
                }
            }
        }

        step
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {

        let nums = vec![2,3,1,1,4];
        let r = Solution::can_jump(nums);
        assert_eq!(2, r);

    }
}
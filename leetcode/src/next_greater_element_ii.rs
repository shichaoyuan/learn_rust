
struct Solution;

impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut stack = Vec::with_capacity(n);
        let mut result: Vec<i32> = vec![-1; n];
        for i in (0..(n*2)).rev() {
            let j = i % n;
            while let Some(&a) = stack.last() {
                if a <= nums[j] {
                    stack.pop();
                } else {
                    break;
                }
            }

            let v = match stack.last() {
                Some(&a) => a,
                None => -1,
            };
            result[j] = v;
            stack.push(nums[j]);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::Solution;

    #[test]
    fn test() {
        let nums = [1,2,3,4,3].to_vec();
        let result = Solution::next_greater_elements(nums);
        assert_eq!([2,3,4,-1,4].to_vec(), result)

    }

}
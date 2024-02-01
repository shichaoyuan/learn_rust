
struct Solution;

impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut track: Vec<i32> = Vec::new();
        Self::backtrack(&nums, 0, &mut track, &mut res);
        res
    }

    fn backtrack(nums: &Vec<i32>, start: usize, track: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        res.push(track.clone());
        for i in start..nums.len() {
            if i > start && nums[i] == nums[i-1] {
                continue;
            }
            track.push(nums[i]);
            Self::backtrack(nums, i+1, track, res);
            track.pop();
        }
    }
}
struct Solution;

impl Solution {
    pub fn video_stitching(clips: Vec<Vec<i32>>, time: i32) -> i32 {
        let mut nums  = vec![0; time as usize];
        for clip in clips.into_iter() {
            if clip[0] < time {
                nums[clip[0] as usize] = nums[clip[0] as usize].max(clip[1]);
            }
        }

        let mut right = 0;
        let mut pre = 0;
        let mut count = 0;
        for i in 0..(time as usize) {
            right = right.max(nums[i]);
            if i == (right as usize) {
                return -1;
            }
            if i == (pre as usize) {
                count += 1;
                pre = right;
            }
        }

        count
    }
}
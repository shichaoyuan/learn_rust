use std::future::IntoFuture;


struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {

        if intervals.is_empty() {
            return 0;
        }

        let n = intervals.len();
        let mut intervals = intervals;
        intervals.sort_by(|a, b| a.get(1).unwrap().cmp(b.get(1).unwrap()));

        let mut count = 1;
        let mut right = intervals[0][1];
        for i in 1..n {
            if intervals[i][0] >= right {
                count += 1;
                right = intervals[i][1];
            }
        }

        (n-count) as i32
    }
}
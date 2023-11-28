struct Solution;

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut points = points;
        points.sort_by(|a, b| a.get(1).unwrap().cmp(b.get(1).unwrap()));

        let mut count = 1;
        let mut right = points[0][1];
        for i in 1..n {
            if points[i][0] > right {
                count += 1;
                right = points[i][1];
            }
        }

        count
    }
}

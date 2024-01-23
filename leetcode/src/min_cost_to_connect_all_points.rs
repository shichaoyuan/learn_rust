
use super::UF;

struct Solution;

impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let mut edges = Vec::new();
        for i in 0..points.len() {
            for j in (i+1)..points.len() {
                let xi = points[i][0];
                let yi = points[i][1];
                let xj = points[j][0];
                let yj = points[j][1];
                edges.push(vec![i, j, ((xi-xj).abs()+(yi-yj).abs()) as usize]);
            }
        }

        edges.sort_by(|a, b| a[2].cmp(&b[2]));

        let mut mst = 0;
        let mut uf = UF::new(points.len());
        for edge in edges.iter() {
            let u = edge[0];
            let v = edge[1];
            let w = edge[2];
            if uf.connected(u, v) {
                continue;
            }

            uf.union(u, v);
            mst += w as i32;
        }

        mst
    }
}
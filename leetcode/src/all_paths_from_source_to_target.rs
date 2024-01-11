
struct Solution;

impl Solution {

    fn traverse(graph: &Vec<Vec<i32>>, i: i32, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        path.push(i);
        if (i as usize) == graph.len()-1 {
            res.push(path.clone());
        }

        for v in graph[i as usize].iter() {
            Self::traverse(graph, *v, path, res);
        }

        path.pop();
    }

    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        Self::traverse(&graph, 0, &mut Vec::new(), &mut res);
        return res;
    }
}
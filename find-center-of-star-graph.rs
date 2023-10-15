use std::collections::{HashMap, HashSet};

pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();

    for edge in &edges {
        let k = edge[0];
        let v = edge[1];

        graph.entry(k).or_insert(HashSet::new()).insert(v);
        graph.entry(v).or_insert(HashSet::new()).insert(k);
    }

    let n = graph.len() as i32;

    for (&node, adj) in &graph {
        if adj.len() == (n - 1) as usize {
            return node;
        }
    }

    -1
}

fn main() {
    println!("{:?}", find_center(vec![vec![1, 2], vec![2, 3], vec![4, 2]]));
    println!("{:?}", find_center(vec![vec![1, 2], vec![5, 1], vec![1, 3], vec![1, 4]]));
}

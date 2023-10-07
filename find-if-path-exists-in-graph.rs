// https://leetcode.com/submissions/detail/1069590987/

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut visited: HashSet<i32> = HashSet::new();
    let mut bfs: VecDeque<i32> = VecDeque::new();

    for edge in &edges {
        let k = edge[0];
        let v = edge[1];

        graph.entry(k).or_insert(Vec::new()).push(v);
        graph.entry(v).or_insert(Vec::new()).push(k);
    }

    bfs.push_back(source);
    visited.insert(source);

    while !bfs.is_empty() {
        if let Some(&current) = bfs.front() {
            bfs.pop_front();

            if current == destination {
                return true;
            }

            if graph.contains_key(&current) {
                for &adj in graph[&current].iter() {
                    if !visited.contains(&adj) {
                        bfs.push_back(adj);
                        visited.insert(adj);
                    }
                }
            }
        }
    }

    false
}

fn main () {
    // Tests
    println!("{:?}", valid_path(3, vec![vec![0,1], vec![1,2], vec![2,0]], 0, 2));
    println!("{:?}", valid_path(6, vec![vec![0,1], vec![0,2], vec![3,5], vec![5,4], vec![4,3]], 0, 5));
    println!("{:?}", valid_path(10, vec![vec![0,7], vec![0,8], vec![6,1], vec![2,0], vec![0,4], vec![5,8], vec![4,7], vec![1,3], vec![3,5], vec![6,5]], 7, 5));
}

// https://leetcode.com/submissions/detail/1075482605/

use std::collections::{HashSet, VecDeque};

pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
    let mut visited: HashSet<i32> = HashSet::new();
    let mut bfs: VecDeque<i32> = VecDeque::new();

    bfs.push_back(0);
    visited.insert(0);

    while !bfs.is_empty() {
        if let Some(current) = bfs.pop_front() {
            for &adj in rooms[current as usize].iter() {
                if !visited.contains(&adj) {
                    bfs.push_back(adj);
                    visited.insert(adj);
                }
            }
        }
    }

    visited.len() == rooms.len()
}

fn main() {
    // Tests
    println!("{:?}", can_visit_all_rooms(vec![vec![1], vec![2], vec![3], vec![]]));
    println!("{:?}", can_visit_all_rooms(vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]]));
}

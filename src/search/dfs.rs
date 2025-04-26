#![allow(non_snake_case)]

use crate::search::GRAPH;

pub fn DFS(start: usize, end: usize) -> Vec<usize> {
    let mut open = vec![vec![start]];
    open.reserve(15);

    while !open.is_empty() {
        let path = open.pop().unwrap();
        let len = path.len();
        let curr = path[len - 1];

        if curr == end {
            return path.clone();
        }

        for (adj, &nb) in GRAPH[curr].iter().rev().enumerate() {
            if nb != 0 {
                if !path.contains(&adj) {
                    let mut np = path.clone();
                    np.push(adj);
                    open.push(np);
                }
            }
        }
    }

    vec![]
}

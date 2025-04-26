#![allow(non_snake_case)]

use crate::{queue::Queue, search::GRAPH};

pub fn BFS(start: usize, end: usize) -> Vec<usize> {
    let mut open = Queue::from_vec(vec![vec![start]]);

    while open.size != 0 {
        let path = open.pop_front_unchecked();
        let len = path.len();
        let curr = path[len - 1];

        if curr == end {
            return path;
        }

        for (adj, &nb) in GRAPH[curr].iter().enumerate() {
            if nb != 0 {
                if !path.contains(&adj) {
                    let mut np = path.clone();
                    np.push(adj);
                    open.push_back(np);
                }
            }
        }
    }

    vec![]
}

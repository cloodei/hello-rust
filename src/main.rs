pub mod utils;
pub mod run;
pub mod queue;
pub mod cli;
pub mod search;

fn main() {
    // run::main();
    println!("DFS: {:?}", search::dfs::DFS(0, 11));
    println!("BFS: {:?}", search::bfs::BFS(0, 11));
}

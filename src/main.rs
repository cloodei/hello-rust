pub mod utils;
pub mod run;
pub mod queue;
pub mod cli;
pub mod search;
pub mod cs;

fn main() {
    // run::main();
    cli::main();
    // println!("DFS: {:?}", search::dfs::DFS(0, 11));
    // println!("BFS: {:?}", search::bfs::BFS(0, 11));

    // cs::server();
}

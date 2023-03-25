// https://stackoverflow.com/questions/59890270/how-do-i-overwrite-console-output
mod graph;
use graph::Graph;
use std::collections::HashMap;

fn main() {
    let (width, height) = (51, 11);
    let mut graph = Graph::from(width, height);

    graph.get_start_end();

    let paths = graph.run_bfs().unwrap_or(HashMap::new());
    if paths.len() == 0 {
        println!("No path was found!");
        return;
    }
    graph.print_path(paths).unwrap();
}

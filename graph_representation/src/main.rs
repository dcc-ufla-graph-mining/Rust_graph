use std::io;

mod representations ;
use representations as rep;

mod canonize;
use canonize as can;


fn main() -> Result<(),Box<dyn std::error::Error>>{
    let graph = rep::Graph::read_graph_from_archive("../../../graphs_ex/cube/".to_string())?;
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let nodes: Vec<usize> = input
        .split_whitespace()
        .filter_map(|part| part.parse().ok())
        .collect();

    let sub_graph = rep::SubGraph::SubGraph::new(nodes, graph);
    println!("{:?}", sub_graph);

    Ok(())
}


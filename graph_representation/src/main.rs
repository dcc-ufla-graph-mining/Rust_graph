use std::io;


mod representations ;
use representations as rep;

mod utils;
mod canonize;


fn main() -> Result<(),Box<dyn std::error::Error>>{
    let graph = rep::Graph::read_graph_from_archive("graphs_ex/other_graph/".to_string())?;
    graph.print_graph();
    
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let nodes: Vec<usize> = input
        .split_whitespace()
        .filter_map(|part| part.parse().ok())
        .collect();
    //println!("\n\n{:?}", nodes);
    let mut sub_graph = rep::SubGraph::SubGraph::new(nodes, &graph);
    
    println!("{:?}\n", sub_graph);
    sub_graph.print_original();
    println!("Value: {}", sub_graph.get_graph_value());
    
    io::stdin().read_line(&mut input)?;

    let _articulation = utils::tarjan::tarjan(sub_graph.get_pattern());
    let mut articulations: Vec<usize> = Vec::new();

    for i in _articulation {
        articulations.push(sub_graph.get_nodes()[i]);
    }
    let new_subgraph = utils::aprimoration::aprimoration(&graph, &sub_graph, articulations).unwrap();
    println!("{:?}", new_subgraph);
    new_subgraph.print_original();
    println!("{}", new_subgraph.get_graph_value());

    Ok(())
}


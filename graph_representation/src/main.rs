use std::io;

mod representations ;
use representations as rep;

mod tarjan;
mod canonize;
//use canonize as can;


fn main() -> Result<(),Box<dyn std::error::Error>>{
    let graph = rep::Graph::read_graph_from_archive("../../graphs_ex/other_graph/".to_string())?;
    println!("{:?}", graph);
    /*
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    let nodes: Vec<usize> = input
        .split_whitespace()
        .filter_map(|part| part.parse().ok())
        .collect();

    let mut sub_graph = rep::SubGraph::SubGraph::new(nodes, graph.clone());
    */
    //sub_graph.print_pattern();
    //println!("{:?}", sub_graph);
    //sub_graph.get_canononical_pattern();
    //println!("{:?}", sub_graph);
    tarjan::tarjan(&graph);
    Ok(())
}


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
    let sub_graph = rep::SubGraph::SubGraph::new(nodes, &graph);
    
    println!("{:?}\n", sub_graph);
    sub_graph.print_original();
    println!("Value: {}", sub_graph.get_graph_value());
    
    io::stdin().read_line(&mut input)?;

    let _articulation = utils::tarjan::tarjan(sub_graph.get_pattern());
    let mut articulations: Vec<usize> = Vec::new();

    for i in _articulation {
        articulations.push(sub_graph.get_nodes()[i]);
    }

    let mut new_subgraph = sub_graph.clone();
    let mut continue_aprimoration: bool = true;
    while continue_aprimoration {
        match utils::aprimoration::aprimoration(&graph, &new_subgraph, articulations.clone(), false) {
            Some(value) => {
                new_subgraph = value;
                println!("Coisas estao acontecenco aqui");
            },
            None => continue_aprimoration = false,
        }
        println!("{:?}", new_subgraph);
        new_subgraph.print_original();
        println!("{}", new_subgraph.get_graph_value());
    }

    Ok(())
}


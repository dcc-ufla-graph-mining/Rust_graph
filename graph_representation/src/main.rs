use rand::prelude::*;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::env;
use std::collections::HashSet;
use std::io;

mod representations ;
use representations as rep;
use rep::SubGraph::SubGraph as Sg;

mod utils;
mod canonize;

fn select_nodes(graph: &rep::Graph::Graph, subgraph_size: usize, rng: &mut SmallRng) -> Vec<usize> {
    // Will stores the nodes for create a subgraph
    let mut nodes: HashSet<usize> = HashSet::new();
    let first_node: usize = rng.gen_range(0..graph.get_num_nodes());

    // Stores the adjacences of the element added as first node of subgraph
    // When new nodes are added, your adjacencies will be stored in this array too
    let mut possible_adjacences: HashSet<usize> = HashSet::new();
    for i in graph.get_adjacencies(first_node) {
        possible_adjacences.insert(i);
    }
    nodes.insert(first_node);
    
    // The for will occur ultil the number of nodes chosed be the same of the 
    // size of node
    while nodes.len() < subgraph_size {
        let mut adjacences_array: Vec<usize> = Vec::new();
        // Transform HashSet in Vec to chose a element in a random position
        for i in possible_adjacences.iter() {
            adjacences_array.push(*i);
        }
        
        adjacences_array.sort();
        let random_position = rng.gen_range(0..adjacences_array.len());
        let random_element: usize = adjacences_array[random_position];
        
        let mut adjacences_random_element: HashSet<usize> = HashSet::new();
        for j in graph.get_adjacencies(random_element) {
            adjacences_random_element.insert(j);
        }
        //Stores de random element chosed
        nodes.insert(random_element);

        // Unite the two sets
        possible_adjacences.extend(adjacences_random_element);
        possible_adjacences.remove(&random_element);
    }
    
    let mut nodes_selected = Vec::new();
    for i in nodes.iter() {
        nodes_selected.push(*i);
    }
    nodes_selected.sort();
    return nodes_selected
}


fn main() -> Result<(),Box<dyn std::error::Error>>{
    let args: Vec<String> = env::args().collect();

    let archive: String = format!("graphs_ex/{}/", args[1].trim_end());
    let subgraph_size: usize = args[2].trim_end().parse()?;
    let seed: u64 = args[3].parse()?;
    let num_subgraphs: usize = args[4].parse()?;
    
    let mut rng = SmallRng::seed_from_u64(seed);

    let graph = rep::Graph::read_graph_from_archive(archive)?;

    let mut best_subgraphs: Vec<Sg> = Vec::new();

    let mut str = String::new();

    /*
    println!("{:?}", graph);
    graph.print_graph();

    let nodes = select_nodes(&graph, subgraph_size, &mut rng);
    let subgraph = Sg::new(nodes, &graph);

    println!("{:?}", subgraph);
    */
    
    for _i in 0..num_subgraphs {
        let nodes = select_nodes(&graph, subgraph_size, &mut rng);
        let sub_graph = Sg::new(nodes,&graph);

        let articulation_pattern = utils::tarjan::tarjan(sub_graph.get_pattern());
        let mut articulations: Vec<usize> = Vec::new();
        for i in articulation_pattern {
            articulations.push(sub_graph.get_nodes()[i]);
        }

        let mut new_subgraph = sub_graph.clone();
        let mut continue_aprimoration: bool = true;
        while continue_aprimoration {
            match utils::aprimoration::aprimoration(&graph, &new_subgraph, articulations.clone(), false) {
                Some(value) => {
                    new_subgraph = value;
                },
                None => continue_aprimoration = false,
            }
        }

        best_subgraphs.push(sub_graph);
        best_subgraphs.push(new_subgraph);
    }

    let mut best_subgraph_index = 1;
    let mut i = 3;
    while i < best_subgraphs.len() {
        if best_subgraphs[i].get_graph_value() > best_subgraphs[best_subgraph_index].get_graph_value() {
            best_subgraph_index = i;
        }
        i += 2;
    }
    println!("{}", best_subgraphs[best_subgraph_index].get_graph_value());
    println!("{:?}", best_subgraphs[best_subgraph_index].get_nodes());
    best_subgraphs[best_subgraph_index].print_pattern();
    
    Ok(())
}


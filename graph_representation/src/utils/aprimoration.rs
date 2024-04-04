#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::collections::HashSet;

use crate::representations;
use representations::Graph::Graph as Gr;
use representations::SubGraph::SubGraph as Sg;


fn gen_subgraph (nodes_aux: &mut Vec<usize>, nodes_removed_and_added: Vec<usize>, graph: &Gr) -> Sg {
        // If this execution generate some improvement

    /*
    nodes_aux.remove(&nodes_removed_and_added[0]);
    nodes_aux.insert(nodes_removed_and_added[1]);
    */
    let unexistent_node = graph.get_num_nodes();

    if nodes_removed_and_added[0] < unexistent_node {
        for (i,j) in nodes_aux.iter().enumerate() {
            if *j == nodes_removed_and_added[0] { nodes_aux.remove(i); break; }
        }
    }
    if nodes_removed_and_added[1] < unexistent_node { nodes_aux.push(nodes_removed_and_added[1]); } 
    /*
    let mut new_nodes: Vec<usize> = Vec::new();

    for i in nodes_aux.iter() {
        new_nodes.push(*i);
    }
    
    new_nodes.sort();*/
    nodes_aux.sort();
    Sg::new(nodes_aux.to_vec(), graph)
}

pub fn aprimoration(graph: &Gr, subgraph: &Sg, articulations: Vec<usize>, first_aprimorator: bool) -> Option<Sg> {
    let mut non_articulation: HashSet<usize> = HashSet::new();
    let mut nodes_aux: Vec<usize> = Vec::new();
    let unexistent_node = graph.get_num_nodes();

    // Store the best pair, being the first the node that will be removed, and the second, the node
    // that will be added
    let mut nodes_removed_and_added: Vec<usize> = vec![unexistent_node, unexistent_node];
    let mut best_subgraph_difference: f64 = subgraph.get_density();
    let first_subgraph_density = best_subgraph_difference;
    println!("Primeira densidade: {first_subgraph_density}");

    for i in subgraph.get_nodes() {
        nodes_aux.push(i);
        non_articulation.insert(i);
    }

    // remove from the set the nodes that is no available to 
    // aprimoration, because is articulation
    for i in &articulations { non_articulation.remove(i); }
/*
    for i in subgraph.get_nodes() {
        let current_difference = try_add_node(subgraph, graph, i);
        if current_difference > best_subgraph_difference {
            nodes_removed_and_added = vec![unexistent_node, i];
        }
    }

    for i in &non_articulation {
        let current_difference = try_remove_node(subgraph, graph, *i);
        if current_difference > best_subgraph_difference {
            nodes_removed_and_added = vec![*i, unexistent_node];
        }
    }
*/

    for i in &non_articulation { // Test remove all the non articulation nodes
        let mut nodes_avaliable: HashSet<usize> = HashSet::new(); // Stores the nodes possible to
                                                                  // expand

        for j in &nodes_aux {
            if *j == *i { continue; }
            for k in &graph.get_adjacencies(*j){
                if !nodes_aux.contains(k) { nodes_avaliable.insert(*k); }
            }
        }

        for j in &nodes_avaliable {

            let current_difference = try_remove_and_add(subgraph, graph, *j, *i);
            if current_difference > best_subgraph_difference{
                nodes_removed_and_added[0] = *i;
                nodes_removed_and_added[1] = *j;
                best_subgraph_difference = current_difference;
                println!("Densidade aprimorante: {current_difference}");
            }
        }
    }

    // If this execution generate some improvement
    if best_subgraph_difference > first_subgraph_density {
        println!("Densidade final e inicial: {} - {}", best_subgraph_difference, first_subgraph_density);
        println!("Nos para adicao e remocao {:?}", nodes_removed_and_added);
        return Some(gen_subgraph(&mut nodes_aux, nodes_removed_and_added, graph));
    }

    None
}

fn try_remove_and_add(subgraph: &Sg, graph: &Gr, node_possible: usize, node_removed: usize) -> f64{
    let subgraph_nodes = subgraph.get_nodes();
    let mut sum_edges_added = 0;
    let mut sum_edges_removed = 0;
    /*
    let value = graph.get_all_edges_value();

    let edges = graph.get_edges(node_removed);
    let adjacencies = graph.get_adjacencies(node_removed);

    for (index, element) in adjacencies.iter().enumerate() {
        if subgraph_nodes.contains(element) { sum_edges_removed += value[edges[index]]; }
    }

    let edges_of_node_possible = graph.get_edges(node_possible);
    let adjacencies = graph.get_adjacencies(node_possible);

    for (index, element) in adjacencies.iter().enumerate() {
        if *element == node_removed { continue; }
        if subgraph_nodes.contains(element) { sum_edges_added += value[edges_of_node_possible[index]]; }
    }
    */
    sum_edges_removed = subgraph.get_pattern().get_edges(subgraph.get_node_index(node_removed)).len();
    for i in graph.get_adjacencies(node_possible) {
        if i == node_removed { continue; }
        if subgraph_nodes.contains(&i) { sum_edges_added += 1; }
    }
    
    let result: f64 = subgraph.get_pattern().get_num_edges() as f64 - sum_edges_removed as f64 + sum_edges_added as f64;
    return (result * 2.0) / (subgraph_nodes.len() as f64 *(subgraph_nodes.len() as f64 - 1.0));
}

fn try_add_node(subgraph: &Sg, graph: &Gr, node_possible: usize) -> f64 {
    let adjacencies = graph.get_adjacencies(node_possible);
    let subgraph_nodes = subgraph.get_nodes();
    let mut sum_of_edges_added: isize = 0;
    
    // This commented part of code is for heavyest subgraph
    /*
    let edges = graph.get_edges(node_possible);
    let value = graph.get_all_edges_value();

    for (index, element) in adjacencies.iter().enumerate() {
        if subgraph_nodes.contains(element) { sum_of_edges_added += value[edges[index]] as isize; }
    }
    */

    // This is for densest subgraph
    for element in &adjacencies {
        if subgraph_nodes.contains(element) { sum_of_edges_added += 1; }
    }

    let result = subgraph.get_pattern().get_num_edges() as f64 + sum_of_edges_added as f64;

    return (result * 2.0) / (subgraph_nodes.len() as f64 *(subgraph_nodes.len() as f64 + 1.0));
}

fn try_remove_node(subgraph: &Sg, graph: &Gr, node_possible: usize) -> f64{
    let subgraph_nodes = subgraph.get_nodes();
    let values = graph.get_all_edges_value();
    let mut sum_of_edges: isize = 0;

    // This part is for heaviest subgraph
    /*
    for i in &subgraph_nodes {
        if *i == node_possible { continue; }
        let edges = graph.get_edges(*i);

        for (index, element) in graph.get_adjacencies(*i).iter().enumerate() {
            if *element != node_possible && subgraph_nodes.contains(element) {
                sum_of_edges += values[edges[index]] as isize;
            }
        }
    }
    */
    let node_index = subgraph.get_node_index(node_possible);
    let edges_removed = subgraph.get_pattern().get_edges(node_index).len() as f64;

    let result = subgraph.get_pattern().get_num_edges() as f64 + edges_removed as f64;

    return (result * 2.0) / ((subgraph_nodes.len() as f64 - 1.0) *(subgraph_nodes.len() as f64 - 2.0));
}

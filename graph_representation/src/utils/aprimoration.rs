use std::collections::HashSet;

use crate::representations;
use representations::Graph::Graph as Gr;
use representations::SubGraph::SubGraph as Sg;


fn gen_subgraph (nodes_aux: &mut HashSet<usize>, nodes_removed_and_added: Vec<usize>, graph: &Gr) -> Sg {
        // If this execution generate some improvement

    nodes_aux.remove(&nodes_removed_and_added[0]);
    nodes_aux.insert(nodes_removed_and_added[1]);
    let mut new_nodes: Vec<usize> = Vec::new();

    for i in nodes_aux.iter() {
        new_nodes.push(*i);
    }
    new_nodes.sort();
    Sg::new(new_nodes, graph)
}

pub fn aprimoration(graph: &Gr, subgraph: &Sg, articulations: Vec<usize>, first_aprimorator: bool) -> Option<Sg> {
    let mut non_articulation: HashSet<usize> = HashSet::new();
    let mut nodes_aux: HashSet<usize> = HashSet::new();

    let mut nodes_removed_and_added: Vec<usize> = vec![subgraph.get_nodes()[0], subgraph.get_nodes()[0]];
    // Store the best pair, being the first the node that will be removed, and the second, the node
    // that will be added

    let mut diff_global: isize = 0;

    for i in subgraph.get_nodes() {
        nodes_aux.insert(i);
    }

    for i in &nodes_aux {
        non_articulation.insert(*i);
    }
    for i in articulations.iter() {
        non_articulation.remove(i);  // remove from the set the nodes that is no available to 
                        // aprimoration, because is articulation
    }

    for i in &non_articulation { // Test remove all the non articulation nodes
        let mut nodes_avaliable: HashSet<usize> = HashSet::new(); // Stores the nodes possible to
                                                                  // expand

        for j in &nodes_aux {
            if *j == *i {
                continue;
            }
            for k in &graph.get_adjacencies(*j){
                if !nodes_aux.contains(k) { nodes_avaliable.insert(*k); }
            }
        }
        
        let mut position = 0;// This value indicates the position of a node wanted
        let mut diff: isize = 0; // Stores the sum of edges removed 
        let values = graph.get_all_edges_value();
        let iterator = subgraph.get_nodes();

        for j in 0..iterator.len() {
            if iterator[j] == *i { position = j; break; }
        }

        for j in subgraph.get_pattern().get_edges(position) {
            diff += values[j] as isize; // Stores the sum of values of edges removed of graph
        }

        for j in &nodes_avaliable {
            let adjacencies = graph.get_adjacencies(*j);
            let edges = graph.get_edges(*j);
            let mut edges_added: Vec<usize> = Vec::new();
            let mut diff_node_added: isize = 0; // Stores the sum of values of edges posible to add
                                                // in graph

            for k in 0..adjacencies.len(){
                if adjacencies[k] == *i{ continue; } // If some node posible to aprimore have the
                                                     // node removed as adjacency, skip that
                                                     // possibility
                if nodes_aux.contains(&adjacencies[k]) {
                    edges_added.push(edges[k]); // Stores the posible edge added
                    diff_node_added += values[edges[k]] as isize;
                }
            }
            
            // If the edges added pays off the cost of edges removed and this difference is bigger
            // than other difference finded before, stores the node removed and the node added
            if diff_node_added - diff > diff_global {
                diff_global = diff_node_added - diff;
                nodes_removed_and_added[0] = *i;
                nodes_removed_and_added[1] = *j;
                if first_aprimorator { return Some(gen_subgraph(&mut nodes_aux, nodes_removed_and_added, graph)); }
           }
       }
    }

    // If this execution generate some improvement
    if nodes_removed_and_added[0] != nodes_removed_and_added[1] {
       return Some(gen_subgraph(&mut nodes_aux, nodes_removed_and_added, graph));
    }

    None
}

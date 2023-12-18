use std::collections::HashSet;

use crate::representations;
use representations::Graph::Graph as Gr;
use representations::SubGraph::SubGraph as Sg;


pub fn aprimoration(graph: &Gr, subgraph: &Sg, articulations: Vec<usize>) {
    let mut non_articulation: HashSet<usize> = HashSet::new();
    let mut nodes_aux: HashSet<usize> = HashSet::new();

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

    println!("Non_articulation: {:?}", non_articulation);

    for i in &non_articulation {
        let mut nodes_avaliable: HashSet<usize> = HashSet::new();

        for j in &nodes_aux {
            if *j == *i {
                continue;
            }
            for k in &graph.get_adjacencies(*j){
                if !nodes_aux.contains(k) { nodes_avaliable.insert(*k); }
            }
        }
        println!("{:?}", nodes_avaliable);
    }
}

use std::collections::HashSet;

use crate::representations;
use representations::Graph::Graph as Gr;
use representations::SubGraph::SubGraph as Sg;


pub fn aprimoration(graph: &Gr, subgraph: &Sg, articulations: Vec<usize>) -> Option<Sg> {
    let mut non_articulation: HashSet<usize> = HashSet::new();
    let mut nodes_aux: HashSet<usize> = HashSet::new();

    let mut nodes_removed_and_added: Vec<usize> = vec![subgraph.get_nodes()[0], subgraph.get_nodes()[0]];
    let mut diff_global: usize = 0;

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

    println!("Non_articulation: {:?}\n", non_articulation);

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
        
        let mut position = 0;
        let mut diff: usize = 0; // Stores the sum of edges removed 
        let values = graph.get_all_edges_value();
        let iterator = subgraph.get_nodes();

        for j in 0..iterator.len() {
            if iterator[j] == *i { position = j; break; }
        }
        print!("Arestas removidas: ");
        for j in subgraph.get_pattern().get_edges(position) {
            print!("{} ", j);
            diff += values[j];
        }
        println!();

        for j in &nodes_avaliable {
            let adjacencies = graph.get_adjacencies(*j);
            let edges = graph.get_edges(*j);
            let mut edges_added: Vec<usize> = Vec::new();
            let mut diff_node_added: usize = 0;

            for k in 0..adjacencies.len(){
                if adjacencies[k] == *i{ continue; }
                if nodes_aux.contains(&adjacencies[k]) {
                    edges_added.push(edges[k]);
                    diff_node_added += values[edges[k]];
                }
            }

            if diff_node_added - diff > diff_global {
                diff_global = diff_node_added - diff;
                nodes_removed_and_added[0] = *i;
                nodes_removed_and_added[1] = *j;
                println!("{:?}", nodes_removed_and_added);


                println!("Remover o vertice {} e adicionar o {},compensa", i, j);
                println!("Arestas adicionadas: {:?}", edges_added);
                print!("Valor das arestas adicionadas: ");
                for k in edges_added.iter() {
                    print!("{} ", values[*k]);
                }
                println!("\nDiferenca: {}", diff_node_added-diff);
                println!("No que gera melhora: {}\n", j);
            }
            else {
                println!("Remover vertice {} e adicionar {} nao compensa ou tem alguma combinação melhor", *i, j);
            }
        }
    }

    println!("{:?}", nodes_removed_and_added);

    if nodes_removed_and_added[0] != nodes_removed_and_added[1] {
        println!("Se ta aqui, eh porque ta certo!");

        nodes_aux.remove(&nodes_removed_and_added[0]);
        nodes_aux.insert(nodes_removed_and_added[1]);
        let mut new_nodes: Vec<usize> = Vec::new();

        for i in nodes_aux {
            new_nodes.push(i);
        }
        new_nodes.sort();
        return Some(Sg::new(new_nodes, graph));
    }

    None
}

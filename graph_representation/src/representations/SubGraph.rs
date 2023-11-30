//use std::io::Error;
use std::collections::HashMap;

use crate::representations::Graph;
use crate::canonize::canon;

//pub mod representations;
#[derive(Debug)]
pub struct SubGraph {
    sizeof: usize,
    nodes: Vec<usize>,
    pattern: Graph::Graph, //pattern
    canonical_pattern: Graph::Graph,
    label_flag: bool,   //flag
    // trocar label por grafo canonico
}

impl SubGraph {
    pub fn new(nodes_of_graph: Vec<usize>, large_graph: Graph::Graph) -> SubGraph{
        let graph = SubGraph::extract_subgraph(large_graph, &nodes_of_graph);
        SubGraph {
            sizeof: nodes_of_graph.len(),
            nodes: nodes_of_graph.clone(),
            pattern: graph,
            canonical_pattern: Graph::Graph::default(),
            label_flag: false,
        }
    }

    pub fn extract_subgraph(graph: Graph::Graph, nodes: &Vec<usize>) -> Graph::Graph{
        let mut start_adj: Vec<usize> = Vec::new();
        let mut adj: Vec<usize> = Vec::new();
        let mut count: usize = 0;

        for i in nodes {
            let adj_aux = graph.get_adjacences(*i);
            start_adj.push(count);
            for j in adj_aux {
                if nodes.contains(&j) {
                    adj.push(j);
                    count += 1;
                }
            }
        }
        
        SubGraph::basic_canonize(&mut adj, nodes.to_vec());
        Graph::Graph::new_filled(start_adj, adj)
    }

    pub fn basic_canonize(adjacences: &mut Vec<usize>, nodes: Vec<usize>) {
        for i in 0..adjacences.len() {
            if let Some((indice, _)) = &nodes
                .iter()
                .enumerate()
                .find(|&(_, &x)| x == i) {
                adjacences[i] = *indice;
            }
        }
    }

    pub fn canon_labeling(graph: Graph::Graph, label: Vec<usize>) -> Graph::Graph {
        let mut map: HashMap<usize, usize> = HashMap::new();
        for i in 0..label.len() {
            map.insert(label[i], i);
        }
        
        let adj_original: Vec<usize> = graph.get_all_adjacences();
        let nodes: Vec<usize> = graph.get_nodes();
        let mut adj_changed: Vec<usize> = vec![0; adj_original.len()];
        let mut nodes_changed = vec![0; nodes.len()];
        let mut nodes_add: usize = 0;

        for i in 0..label.len() {
            let aux = get_adjacences(&adj_original, &nodes, label[i]);
            nodes_changed[i] = nodes_add;

            for j in aux {
                if let Some(value) = map.get(&j) {
                    adj_changed[nodes_add] = *value;
                    nodes_add += 1;
                }
                else {
                    break;
                }
            }
        }

        graph
    }

    pub fn get_canononical_pattern(&self) -> Graph::Graph {
        if self.label_flag {
            return self.canonical_pattern.clone();
        }
        
        let canonical = match canon::canon(&self.pattern) {
            Ok(canonical) => { 
                let mut new_subgraph: Graph::Graph = self.canonical_pattern.clone();
                return SubGraph::canon_labeling(self.canonical_pattern.clone(), canonical);
            },
            Err(_) => {
                return Graph::Graph::default();
            }
        };
    }
}

fn get_adjacences(adj: &Vec<usize>, nodes: &Vec<usize>, node: usize) -> Vec<usize> {
    let mut aux: Vec<usize> = Vec::new();
    //let i = self.nodes[node];
    for i in nodes[node]..nodes[node+1] {
        aux.push(adj[i]);
    }
    aux
}



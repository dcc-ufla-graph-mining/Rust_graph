//use std::io::Error;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::representations::Graph;
use crate::canonize::canon;

//pub mod representations;
#[derive(Debug)]
#[derive(Clone)]
pub struct SubGraph {
    sizeof: usize,
    nodes: Vec<usize>,
    pattern: Graph::Graph, //pattern
    canonical_pattern: Graph::Graph,
    label_flag: bool,   //flag
    // trocar label por grafo canonico
}

#[allow(dead_code)]
impl SubGraph {
    pub fn new(nodes_of_graph: Vec<usize>, large_graph: &Graph::Graph) -> SubGraph{
        let graph = SubGraph::extract_subgraph(large_graph, &nodes_of_graph);
        SubGraph {
            sizeof: nodes_of_graph.len(),
            nodes: nodes_of_graph,
            pattern: graph,
            canonical_pattern: Graph::Graph::default(),
            label_flag: false,
        }
    }


    /*
     *  Extract the subgraph, using some nodes defined by program or user.
     *  Create a graph for represent the subgraph.
     *  Also transform the subgraph in his self pattern
     */
    pub fn extract_subgraph(graph: &Graph::Graph, nodes: &Vec<usize>) -> Graph::Graph{
        let mut start_adj: Vec<usize> = Vec::new(); // Stores the start position of a adjacencie
        let mut adj: Vec<usize> = Vec::new(); // Stores the adjacencies
        let mut edg: Vec<usize> = Vec::new();
        let mut edg_value: Vec<usize> = Vec::new();
        let mut count: usize = 0;
        let all_edges = graph.get_all_edges_value();

        for i in nodes {
            let adj_aux = graph.get_adjacencies(*i);
            let edg_aux = graph.get_edges(*i);
            start_adj.push(count);
            for j in 0..adj_aux.len() {
                if nodes.contains(&adj_aux[j]) {
                    adj.push(adj_aux[j].clone());
                    edg.push(edg_aux[j].clone());
                    count += 1;
                }
            }
        }
        start_adj.push(adj.len());

        let mut hash: HashMap<usize,usize> = HashMap::new();
        let mut index = 0;
        for i in 0..edg.len() {
            if let Some(valor) = hash.get(&edg[i]) { continue; }
            else { hash.insert(edg[i], index); index += 1; }
        }

        for (key, element) in &hash {
            edg_value.push(all_edges[*key]);
        }

        for i in 0..edg.len() {
            edg[i] = *hash.get(&edg[i]).unwrap();
        }

        SubGraph::set_pattern(&mut adj, nodes);
        Graph::Graph::new_filled(start_adj, adj, edg, edg_value)
    }

    /*
     *
     *  Take the nodes of subgraph and make a translation,
     *  as if it were a independent graph.
     *
     */
    pub fn set_pattern(adjacencies: &mut Vec<usize>, nodes: &Vec<usize>) {
        let mut map: HashMap<usize, usize> = HashMap::new();
        for i in 0..nodes.len() {
            map.insert(nodes[i], i);
        }

        for i in 0..adjacencies.len() {
            let _aux = adjacencies[i];
            adjacencies[i] = *map.get(&_aux).unwrap();
        }
    }

    /*
     *
     *  Returns a reference to subgraph pattern
     *
     */
    
    pub fn get_pattern(&self) -> &Graph::Graph {
        &self.pattern
    }

    /*
     *
     *  Use the canonical label getted from nauty_traces and 
     *  canonize the graph.
     *
     */
    pub fn canon_labeling(&self, label: Vec<usize>) -> Graph::Graph {
        let mut map: HashMap<usize, usize> = HashMap::new();
        for i in 0..label.len() {
            map.insert(label[i], i);
        }
        
        let adj_original: Vec<usize> = self.pattern.get_all_adjacencies(); // Stores the original order of 
                                                                           // nodes.
        let nodes: Vec<usize> = self.pattern.get_nodes();   // Stores the nodes corresponding in
                                                            // original graph
        let mut adj_changed: Vec<usize> = vec![0; adj_original.len()]; // Used for construct the
                                                                       // new graph.
                                                                       // The adjacence list will
                                                                       // be put here
        let mut nodes_changed = vec![0; nodes.len()];   // Will stores the start of adjacency of a
                                                        // node.
        let mut nodes_add: usize = 0;

        for i in 0..label.len() {
            // let aux = get_adjacencies(&adj_original, &nodes, label[i]);
            let aux = self.pattern.get_adjacencies(label[i]);
            nodes_changed[i] = nodes_add;

            for j in aux {
                adj_changed[nodes_add] = *map.get(&j).unwrap();
                nodes_add += 1;
            }
        }

        Graph::Graph::new_filled(nodes_changed, adj_changed, vec![], vec![]) // Construct the new
                                                                             // graph canonized
    }

    /*
     *  Returns the canonical pattern if was calculated
     *  If not, calculate and return
     */
    pub fn get_canononical_pattern(&mut self) -> Graph::Graph {
        if self.label_flag {
            return self.canonical_pattern.clone();
        }
        
        let _canonical = match canon::canon(&self.pattern) {
            Ok(_canonical) => { 
                self.canonical_pattern = self.canon_labeling(_canonical);
                self.label_flag = true;
            },
            Err(_) => {
                println!("Erro ao calcular o rótulo canônico");
                return Graph::Graph::default();
            }
        };
        self.canonical_pattern.clone()
    }

    pub fn print_pattern(&self) {
        self.pattern.print_graph();
    }

    pub fn print_original(&self) {
        let mut map: HashMap<usize, usize> = HashMap::new();
        for i in 0..self.sizeof {
            map.insert(i, self.nodes[i]);
        }

        for i in 0..self.sizeof {
            let aux = self.pattern.get_adjacencies(i);
            print!("{} - ", self.nodes[i]);

            for j in aux {
                print!("{} ", *map.get(&j).unwrap());
            }
            println!();
        }
    }

    pub fn get_graph_value(&self) -> usize {
        let mut value: usize = 0;
        for i in self.pattern.get_all_edges_value() {
            value += i;
        }
        value
    }

    pub fn get_nodes(&self) -> Vec<usize> {
        self.nodes.clone()
    }

    pub fn get_node_index(&self, node: usize) -> usize {
        let mut wanted_node = 0;
        for i in 0..self.nodes.len() {
            if self.nodes[i] == node { wanted_node = i; break; }
        }
        wanted_node
    }

    pub fn get_density(&self) -> f64{
        (self.get_pattern().get_num_edges() as f64 * 2.0) / (self.get_nodes().len() as f64 * (self.get_nodes().len() as f64 - 1.0))
    }
}


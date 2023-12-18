use std::io::{BufRead, BufReader, Error, Read};
use std::fs::File;


#[derive(Debug)]
#[derive(Default)]
#[derive(Clone)]
pub struct Graph {
    num_nodes: usize,
    num_edges: usize,
    adjacencies: Vec<usize>,
    edges: Vec<usize>,
    edges_value: Vec<usize>,
    nodes: Vec<usize>,
}

impl Graph {
    pub fn new(nodes: usize, edges: usize) -> Graph{
        let mut _nodes = vec![0; nodes+1];
        _nodes[nodes] = 2*edges;
        Graph { 
            num_nodes: nodes,
            adjacencies: Vec::with_capacity(2*edges),
            edges: Vec::with_capacity(2*edges),
            edges_value: Vec::with_capacity(edges),
            nodes: _nodes,
            num_edges: edges,
        }
    }

    pub fn new_filled(start_adj: Vec<usize>, _adjacencies: Vec<usize>, _edges: Vec<usize>, _edges_value: Vec<usize>) -> Graph {
        Graph {
            num_nodes: start_adj.len()-1,
            adjacencies: _adjacencies.clone(),
            edges: _edges,
            edges_value: _edges_value,
            nodes: start_adj,
            num_edges: _adjacencies.len(),
        }
    }

    fn set_edge_value(&mut self, edge_value: Vec<usize>) {
        for i in 0..edge_value.len() {
            self.edges_value.push(edge_value[i]);
        }
    }

    pub fn add_node(&mut self, node: usize, adjacencies_of_node: Vec<usize>, e_label: Vec<usize>, last_position: usize) -> usize {
        self.nodes[node] = last_position;

        if adjacencies_of_node.len() > 0 {
            for i in 0..adjacencies_of_node.len() {
                self.adjacencies.push(adjacencies_of_node[i]);
                self.edges.push(e_label[i]);
            }
        }
        
        adjacencies_of_node.len()
    }

    pub fn print_graph(&self) {
        for i in 0..self.num_nodes{
            print!("{} - ", i);

            for j in self.nodes[i]..self.nodes[i+1]{
                print!("{},{} ", self.adjacencies[j], self.edges_value[self.edges[j]]);
            }
            println!();
        }
    }

    pub fn print_nodes(&self) {
        println!("{:?}", self.nodes);
    }

    pub fn get_adjacencies(&self, node: usize) -> Vec<usize> {
        let mut adj: Vec<usize> = Vec::new();
        //let i = self.nodes[node];
        //println!("{:?}", self);
        for i in self.nodes[node]..self.nodes[node+1] {
            adj.push(self.adjacencies[i].clone());
        }
        adj
    }

    pub fn get_edges(&self, node:usize) -> Vec<usize> {
        let mut edg: Vec<usize> = Vec::new();

        for i in self.nodes[node]..self.nodes[node+1] {
            edg.push(self.edges[i].clone());
        }
        edg
    }

    pub fn get_all_edges(&self) -> Vec<usize> {
        self.edges.clone()
    }

    pub fn get_all_adjacencies(&self) -> Vec<usize> {
        self.adjacencies.clone()
    }

    pub fn get_nodes(&self) -> Vec<usize> {
        self.nodes.clone()
    }

    pub fn get_num_nodes(&self) -> usize {
        let num_nodes = self.num_nodes;
        num_nodes
    }

    pub fn get_all_edges_value(&self) -> Vec<usize> {
        self.edges_value.clone()
    }

    pub fn get_edge_value(&self, node: usize) -> usize {
        self.edges_value[node]
    }
}



/*
 * Read a Graphs from some archives in a formated pattern
 */
pub fn read_graph_from_archive (archives_path: String) -> Result<Graph, Error>{
    let metadata = format!("{}{}", archives_path, String::from("metadata"));
    //println!("{}", metadata);
    let mut file = match File::open(&metadata){
        Ok(file) => file,
        Err(_err) => {
            println!("Error in find the archive."); 
            return Err(_err);
        }
    };
    
    /*
     *
     * Take a line in metadata, transform in a vector of strings and
     * after transform in a vector of usize
     * In the end, create the graph with informations:
     * number of nodes and number of edges
     *
     */
    let mut line = String::new();
    file.read_to_string(&mut line)?;
    let line: Vec<&str> = line
        .trim_end()
        .split_whitespace()
        .collect();
    let features: Vec<usize> = line
        .iter()
        .map(|&s| s.parse().unwrap())
        .collect();

    let mut graph: Graph = Graph::new(features[0], features[1]);

    /*
     *
     *  Run in adjlists archive, taking each line, transforming in a vector 
     *  of string, transforming into another vector of strings for access only
     *  the first element, and in the end take the first element of the vector
     *  and transform in a usize for ading as an element adjacencie of the node
     *
     */
    let adjlists = format!("{}{}", archives_path, String::from("adjlists"));
    let file = match File::open(&adjlists){
        Ok(file) => file,
        Err(_err) => {
            println!("{}\nLeitura falha", _err); 
            return Err(_err);
        }
    };
    let reader = BufReader::new(&file);
    let mut last_node_position = 0;

    for (v, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            let line: Vec<&str> = line
                .trim_end()
                .split_whitespace()
                .collect();

            let mut adjacencies: Vec<usize> = Vec::new();
            let mut e_label: Vec<usize> = Vec::new();
            for i in &line {
                let i: Vec<&str> = i
                    .split(",")
                    .collect();
                adjacencies.push(i[0].parse().unwrap());
                e_label.push(i[1].parse().unwrap());
            }
            last_node_position += graph.add_node(v, adjacencies, e_label, last_node_position);
        }
    }

    /*
     *  Run in elabels archive, storing in a vector
     *  the value of the edges.
     */
    let labels = format!("{}{}", archives_path, String::from("elabels"));
    let file = File::open(&labels).unwrap();
    let reader = BufReader::new(&file);
    let mut edge_value: Vec<usize> = Vec::new();

    for (_, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            let line: usize = line.trim_end().parse().unwrap();

            edge_value.push(line);
        }
    }

    graph.set_edge_value(edge_value); // Stores graph struct the value of edges

    Ok(graph)
}


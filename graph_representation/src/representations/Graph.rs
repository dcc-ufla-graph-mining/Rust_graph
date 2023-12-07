use std::io::{BufRead, BufReader, Error, Read};
use std::fs::File;


#[derive(Debug)]
#[derive(Default)]
#[derive(Clone)]
pub struct Graph {
    num_nodes: usize,
    adjacences: Vec<usize>,
    edges: Vec<usize>,
    nodes: Vec<usize>,
    num_of_nodes_add: usize,
    num_edges: usize,
}

impl Graph {
    pub fn new(nodes: usize, edges: usize) -> Graph{
        let mut _nodes = vec![0; nodes+1];
        _nodes[nodes] = 2*edges;
        Graph { 
            num_nodes: nodes,
            adjacences: Vec::with_capacity(2*edges),
            edges: Vec::with_capacity(2*edges),
            nodes: _nodes,
            num_of_nodes_add: 0,
            num_edges: edges,
        }
    }

    pub fn new_filled(start_adj: Vec<usize>, _adjacences: Vec<usize>, _edges: Vec<usize>) -> Graph {
        Graph {
            num_nodes: start_adj.len()-1,
            adjacences: _adjacences.clone(),
            edges: _edges,
            nodes: start_adj,
            num_of_nodes_add: _adjacences.len(),
            num_edges: _adjacences.len(),
        }
    }

    fn change_edge_value(&mut self, edge_value: Vec<usize>) {
        println!("{:?}\n{:?}", self.edges, edge_value);
        for i in 0..self.edges.len() {
            self.edges[i] = edge_value[self.edges[i]];
        }
    }

    pub fn add_node(&mut self, node: usize, adjacences_of_node: Vec<usize>, e_label: Vec<usize>) {
        if adjacences_of_node.len() > 0 {
            //self.nodes.push(self.num_of_nodes_add);
            self.nodes[node] = self.num_of_nodes_add;
            
            for i in 0..adjacences_of_node.len() {
                self.adjacences.push(adjacences_of_node[i]);
                self.edges.push(e_label[i]);
            }
        }
        else {
            self.nodes[node] = self.num_of_nodes_add;
        }
        self.num_of_nodes_add += adjacences_of_node.len();
        
    }

    pub fn print_graph(&self) {
        for i in 0..self.num_nodes{
            print!("{} - ", i);

            for j in self.nodes[i]..self.nodes[i+1]{
                print!("{} ", self.adjacences[j]);
            }
            println!();
        }
    }

    pub fn print_nodes(&self) {
        println!("{:?}", self.nodes);
    }

    pub fn get_adjacences(&self, node: usize) -> Vec<usize> {
        let mut adj: Vec<usize> = Vec::new();
        //let i = self.nodes[node];
        //println!("{:?}", self);
        for i in self.nodes[node]..self.nodes[node+1] {
            adj.push(self.adjacences[i].clone());
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

    pub fn get_all_adjacences(&self) -> Vec<usize> {
        self.adjacences.clone()
    }

    pub fn get_nodes(&self) -> Vec<usize> {
        self.nodes.clone()
    }

    pub fn get_num_nodes(&self) -> usize {
        let num_nodes = self.num_nodes;
        num_nodes
    }
}

/*
 * Read a Graphs from some archives in a formated pattern
 */
pub fn read_graph_from_archive (archives_path: String) -> Result<Graph, Error>{
    let metadata = format!("{}{}", archives_path, String::from("metadata"));
    println!("{}", metadata);
    let mut file = match File::open(&metadata){
        Ok(file) => file,
        Err(_err) => {
            println!("{}", _err); 
            return Err(_err);
        }
    };
    
    // Take a line in metadata, transform in a vector of strings and
    // after transform in a vector of usize
    // In the end, create the graph with informations:
    // number of nodes and number of edges
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

    // Run in adjlists archive, taking each line, transforming in a vector 
    // of string, transforming into another vector of strings for access only
    // the first element, and in the end take the first element of the vector
    // and transform in a usize for ading as an element adjacence of the node
    let adjlists = format!("{}{}", archives_path, String::from("adjlists"));
    let file = match File::open(&adjlists){
        Ok(file) => file,
        Err(_err) => {
            println!("{}\nLeitura falha", _err); 
            return Err(_err);
        }
    };
    let reader = BufReader::new(&file);

    for (v, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            let line: Vec<&str> = line
                .trim_end()
                .split_whitespace()
                .collect();

            let mut adjacences: Vec<usize> = Vec::new();
            let mut e_label: Vec<usize> = Vec::new();
            for i in &line {
                let i: Vec<&str> = i
                    .split(",")
                    .collect();
                adjacences.push(i[0].parse().unwrap());
                e_label.push(i[1].parse().unwrap());
            }
            graph.add_node(v, adjacences, e_label);

        }
    }
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
    println!("{:?}", edge_value);
    graph.change_edge_value(edge_value);

    Ok(graph)
}
/*
fn main() {
    let mut archive = String::new();
    println!("Input the name of the directory where are the informations about the graph:");
    io::stdin().read_line(&mut archive).expect("");
    //println!("{}", archive);
    let path = format!("{}{}{}", String::from("../../graphs_ex/"), archive.trim_end(), String::from("/"));
    let read_graph = read_graph_from_archive(path);
    let mut graph: Graph = Graph::default();
    if let Ok(value) = read_graph {
        graph = value;
    }
    graph.print_graph();
    //graph.print_nodes();
}*/

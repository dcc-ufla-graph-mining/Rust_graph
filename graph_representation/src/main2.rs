use std::io;
use std::io::{BufRead, BufReader, Error, Read};
use std::fs::File;

#[derive(Debug)]
#[derive(Default)]
struct Graph {
    num_nodes: usize,
    adjacences: Vec<usize>,
    nodes: Vec<usize>,
    num_of_nodes_add: usize,
}

impl Graph {
    fn new(nodes: usize, edges: usize) -> Graph{
        Graph { 
            num_nodes: nodes,
            adjacences: Vec::with_capacity(edges),
            nodes: Vec::with_capacity(nodes),
            num_of_nodes_add: 0 
        }
    }

    fn add_node(&mut self, adjacences_of_node: Vec<usize>) {
        println!("{:?}", adjacences_of_node);
        self.num_of_nodes_add += adjacences_of_node.len();
        if adjacences_of_node.len() == 0 {
            self.nodes.push(self.nodes[self.nodes.len()-1]);
        }
        else {
            self.nodes.push(self.num_of_nodes_add-1);
            
            for i in adjacences_of_node {
                self.adjacences.push(i);
            }
        }
    }


    fn print_graph(&self) {
        let mut begin: usize = 0;
        for i in 0..self.num_nodes{
            print!("{} - ", i);

            for j in begin..=self.nodes[i]{
                print!("{} ", self.adjacences[j]);
            }
            println!();
            begin = (self.nodes[i]+1) as usize;
        }
    }
}

/*
 * Read a Graphs from some archives in a formated pattern
 */
fn read_graph_from_archive (archives_path: String) -> Result<Graph, Error>{
    let metadata = format!("{}{}", archives_path, String::from("metadata"));
    println!("{}", metadata);
    let mut file = match File::open(&metadata){
        Ok(file) => file,
        Err(_err) => {
            println!("{}111", _err); 
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
            println!("{}111", _err); 
            return Err(_err);
        }
    };
    let reader = BufReader::new(&file);

    for (_, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            let line: Vec<&str> = line
                .trim_end()
                .split_whitespace()
                .collect();

            let mut adjacences: Vec<usize> = Vec::new();
            for i in &line {
                let i: Vec<&str> = i
                    .split(",")
                    .collect();
                match i[0].parse::<usize>() {
                    Ok(value) => {println!("{}NADA", value);adjacences.push(value);},
                    Err(_err) => {println!("{}", _err);},
                }
            }
            graph.add_node(adjacences);

        }
    }
    Ok(graph)
}

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
}

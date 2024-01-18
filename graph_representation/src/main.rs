use std::io;
use std::thread;
use rand::random;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

mod representations ;
use representations as rep;
use rep::SubGraph::SubGraph as Sg;

mod utils;
mod canonize;

fn select_nodes(graph: &rep::Graph::Graph, subgraph_size: usize) -> Vec<usize> {
    // Will stores the nodes for create a subgraph
    let mut nodes: HashSet<usize> = HashSet::new();
    let first_node: usize = random::<usize>() % graph.get_num_nodes();

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

        let random_position = random::<usize>() % adjacences_array.len();
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
    return nodes_selected
}


fn main() -> Result<(),Box<dyn std::error::Error>>{
    let graph = rep::Graph::read_graph_from_archive("graphs_ex/other_graph/".to_string())?;
    graph.print_graph();
    
    let mut input = String::new();
    println!("Numero de nos que conterá o subgrafo: ");
    io::stdin().read_line(&mut input)?;
    let subgraph_size: usize = input.trim_end().parse()?;
    input.clear();
    println!("Numero de threads que serao usadas:");
    io::stdin().read_line(&mut input)?;
    println!("\n{}", input);
    let num_threads:usize = input.trim_end().parse()?;

    let mut handles = Vec::new();
    let best_locals = Arc::new(Mutex::new(Vec::new()));
    let graph_arc = Arc::new(graph.clone());

    for _thread_index in 0..num_threads {
        //println!("oi");
        // Use this to enable the use in new thread
        // let graph_reference = &graph;
        let mutex = Arc::clone(&best_locals);
        let graph_clone = graph_arc.clone();

        // Spawn a new thread that searches a subgraph and improves
        let handle = thread::spawn(move || {
            let nodes = select_nodes(&graph_clone, subgraph_size);
            let sub_graph = Sg::new(nodes,&graph_clone);
            
            let articulation_pattern = utils::tarjan::tarjan(sub_graph.get_pattern());
            let mut articulations: Vec<usize> = Vec::new();
            for i in articulation_pattern {
                articulations.push(sub_graph.get_nodes()[i]);
            }

            let mut new_subgraph = sub_graph.clone();
            let mut continue_aprimoration: bool = true;
            while continue_aprimoration {
                match utils::aprimoration::aprimoration(&graph_clone, &new_subgraph, articulations.clone(), false) {
                    Some(value) => {
                        new_subgraph = value;
                        //println!("Coisas estao acontecenco aqui");
                    },
                    None => continue_aprimoration = false,
                }
                //println!("{:?}", new_subgraph);
                //new_subgraph.print_original();
                //println!("{}", new_subgraph.get_graph_value());
            }
            let mut vec_aux = mutex.lock().unwrap();
            vec_aux.push(sub_graph);
            vec_aux.push(new_subgraph);
            /*
            let mut vec_muttable = mutex_vec.lock().unwrap();
            vec_muttable.push(new_subgraph);*/
        });

        // Store the thread, for use the join function
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    let best_subgraphs = best_locals.lock().unwrap().to_vec();
    let mut i = 0;
    while i < best_subgraphs.len() {
        println!("{:?}\n{:?}\n\n", best_subgraphs[i], best_subgraphs[i+1]);
        i += 2;
    }

    /*
     * Was the user input of nodes for the subgraph
    io::stdin().read_line(&mut input)?;
    let nodes: Vec<usize> = input
        .split_whitespace()
        .filter_map(|part| part.parse().ok())
        .collect();
    */
    //println!("\n\n{:?}", nodes);
    /*
    let nodes: Vec<usize> = vec![0];
    let sub_graph = Sg::new(nodes, &graph);
    
    println!("{:?}\n", sub_graph);
    sub_graph.print_original();
    println!("Value: {}", sub_graph.get_graph_value());
    
    io::stdin().read_line(&mut input)?;

    let _articulation = utils::tarjan::tarjan(sub_graph.get_pattern());
    let mut articulations: Vec<usize> = Vec::new();

    for i in _articulation {
        articulations.push(sub_graph.get_nodes()[i]);
    }

    let mut new_subgraph = sub_graph.clone();
    let mut continue_aprimoration: bool = true;
    while continue_aprimoration {
        match utils::aprimoration::aprimoration(&graph, &new_subgraph, articulations.clone(), false) {
            Some(value) => {
                new_subgraph = value;
                println!("Coisas estao acontecenco aqui");
            },
            None => continue_aprimoration = false,
        }
        println!("{:?}", new_subgraph);
        new_subgraph.print_original();
        println!("{}", new_subgraph.get_graph_value());
    }
    */
    Ok(())
}


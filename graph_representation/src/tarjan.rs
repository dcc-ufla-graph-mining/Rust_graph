use crate::representations::Graph;
use Graph::Graph as Gr;

fn DFS(graph: &Gr, discover_time: &mut Vec<isize>, low: &mut Vec<isize>, stack: &mut Vec<usize>, node: usize, mut counter: isize) {
    for i in graph.get_adjacences(node) {
        //println!("{}", i);
        if low[i] == -1 {
            discover_time[i] = counter;
            low[i] = counter;
            stack.push(i);
            counter += 1;

            DFS(graph, discover_time, low, stack, i, counter);

            if low[i] < low[node] { low[node] = low[i]; }
        }
    }
}

pub fn tarjan(graph: &Gr/*, large_graph: Gr*/) {
    let mut discover_time: Vec<isize> = vec![-1; graph.get_num_nodes()];
    let mut low: Vec<isize> = vec![-1; graph.get_num_nodes()];
    let mut stack: Vec<usize> = Vec::new();
    let global_counter = 1;

    discover_time[0] = 0;
    low[0] = 0;
    DFS(&graph, &mut discover_time, &mut low, &mut stack, 0, global_counter);
    println!("{:?}\n{:?}", discover_time, low);
    for i in 0..graph.get_num_nodes() {
        for j in graph.get_adjacences(i) {
            if low[j] >= discover_time[i] { println!("opa"); }
        }
    }
}

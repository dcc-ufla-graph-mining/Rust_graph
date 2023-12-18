use crate::representations::Graph;
use Graph::Graph as Gr;

fn DFS(graph: &Gr, discover_time: &mut Vec<isize>, low: &mut Vec<isize>, node: usize, mut counter: isize, parent: isize, articulation: &mut Vec<usize>) {
    discover_time[node] = counter;
    low[node] = counter;
    counter += 1;
    let mut children = 0;

    for i in graph.get_adjacencies(node) {
        if low[i] == -1 {
            children += 1;

            DFS(graph, discover_time, low, i, counter, node as isize, articulation);

            if low[i] < low[node] { low[node] = low[i]; }
            if parent != -1 && low[i] >= discover_time[node] {
                articulation.push(node);
            }

        }
        else if i as isize != parent{
            if low[node] > discover_time[i] { low[node] = discover_time[i]; }
        }
    }
    if parent == -1 && children > 1 {
        articulation.push(node);
    }
}

pub fn tarjan(graph: &Gr) -> Vec<usize> {
    let num_nodes = graph.get_num_nodes();
    let mut discover_time: Vec<isize> = vec![-1; num_nodes];
    let mut low: Vec<isize> = vec![-1; num_nodes];
    let mut articulation: Vec<usize> = Vec::new();
    let global_counter = 0;

    discover_time[0] = 0;
    low[0] = 0;
    DFS(&graph, &mut discover_time, &mut low, 0, global_counter, -1, &mut articulation);
    articulation
}

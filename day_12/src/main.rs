extern crate petgraph;
extern crate regex;

use std::fs::File;
use std::io::Read;
use petgraph::Graph;
use petgraph::graph::NodeIndex;
use regex::Regex;

use petgraph::algo::{min_spanning_tree, connected_components, kosaraju_scc};

fn task_1() {
    let mut f = File::open("./input").expect("file not found");
    // let mut f = File::open("./test_1").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something wrong happened reading file");

    let (nodes, g) = graph_from_input(&contents);
    let result = kosaraju_scc(&g);        
    let num_connected_nodes: usize = result.iter()
                                            .filter(|mut x| x.contains(&nodes[0]))
                                            .map(|x| x.len())
                                            .sum();

    println!("Task 1 results: {}", num_connected_nodes);

    // The number of strongly connected components are the number of requested "groups"
    println!("Task 2 results: {}", result.len());

}

fn graph_from_input(input: &str) -> (Vec<NodeIndex>, Graph<usize, usize, petgraph::Undirected>) {
    let mut g = Graph::<usize, usize, petgraph::Undirected>::new_undirected();
    let weight = 0; // needed for petgraph, but unused for our graph
    let mut nodes = Vec::new();

    for i in 0..input.trim().lines().count() {
        nodes.push(g.add_node(i)); // Add nodes have the same weight as well
    }

    // 0 <-> 1, 2, 3
    let line_regex = Regex::new("([0-9]*) <-> (.*)").unwrap();

    for line in input.trim().lines() {
        for m in line_regex.captures_iter(line) {
            for end_node in m[2].split(", ") {
                let end_node_index = end_node.parse::<usize>().ok().unwrap();
                let first_node_index = m[1].parse::<usize>().ok().unwrap();
                g.add_edge(nodes[first_node_index], nodes[end_node_index], 0);
            }
        }
    }

    (nodes, g)
}

fn main() {
    task_1();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_lengths() {
        let INPUT = "0 <-> 2
        1 <-> 1
        2 <-> 0, 3, 4
        3 <-> 2, 4
        4 <-> 2, 3, 6
        5 <-> 6
        6 <-> 4, 5";
        let (nodes, g) = graph_from_input(INPUT);
        let result = kosaraju_scc(&g);        
        let num_connected_nodes: usize = result.iter()
                                               .filter(|mut x| x.contains(&nodes[0]))
                                               .map(|x| x.len())
                                               .sum();
        assert_eq!(num_connected_nodes, 6);
    }
}

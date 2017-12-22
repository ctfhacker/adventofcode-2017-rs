extern crate indextree;
extern crate regex;

use indextree::Arena;
use indextree::NodeId;
use regex::Regex;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    name: String,
    weight: u32,
}

impl Node {
    fn new(name: String, weight: u32) -> Node {
        Node {
            name: name,
            weight: weight,
        }
    }
}

fn total_node_weight(node: NodeId, arena: &Arena<Node>) -> u32 {
    let mut weight = arena[node].data.weight;
    for child in node.children(arena) {
        weight += arena[child].data.weight;
        for second_child in child.children(arena) {
            weight += total_node_weight(second_child, arena);
        }
    }
    weight
}

fn calculate_weights(node: NodeId, arena: &Arena<Node>) {
    let node_weights = node.children(arena).map(|c| total_node_weight(c, arena)).collect::<Vec<u32>>();

    if node_weights.len() == 0 {
        // Terminal node
        return;
    }

    if node_weights.iter().all(|&x| x == (node_weights.iter().sum::<u32>() / (node_weights.iter().count() as u32))) {
        // println!("Perfect Node: {:?}", node_weight);
        for child in node.children(arena) {
            calculate_weights(child, arena);
        }
    } else {
        println!("Sum: {} each: {}", node_weights.iter().sum::<u32>(), node_weights.iter().sum::<u32>() / (node_weights.iter().count() as u32));
        println!("{} ({}) -- Bad Node: {:?}", arena[node].data.name, arena[node].data.weight, node_weights);
        for child in node.children(arena) {
            calculate_weights(child, arena);
        }
    }
}

fn main() {
    let mut f = File::open("./input").expect("file not found");
    // let mut f = File::open("./test").expect("file not found");
    let line_regex = Regex::new("([a-z]*) \\((.*)\\)").unwrap();
    let children_regex = Regex::new("-> (.*)$").unwrap();
    let mut nodes = HashMap::new();
    let arena = &mut Arena::new();

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something wrong happened reading file");

    // First run for setting all nodes
    for line in contents.lines() {
        for matches in line_regex.captures_iter(line) {
            let name = matches[1].to_string();
            let weight = matches[2].parse::<u32>().ok().unwrap();
            let new_node = arena.new_node(Node::new(name.clone(), weight));
            println!("{} -> {:?}", name, new_node);
            nodes.insert(name, new_node);
        }
    }

    let mut last_child = None;

    // Second pass for setting children
    for line in contents.lines() {
        let mut name = String::from("");
        for matches in line_regex.captures_iter(line) {
            name = matches[1].to_string();
        }

        let parent = nodes.get(&name).expect("Could not get parent node");

        for matches in children_regex.captures_iter(line) {
            for child_name in matches[1].split(", ") {
                let child = nodes.get(child_name).expect("Could not get child node");
                parent.append(*child, arena);
                // println!("{} -> {:?}", name, child);
                last_child = Some(child);
            }
        }
    }

    if let Some(node) = last_child {
        // println!("{:?}", node.ancestors(arena).collect::<Vec<NodeId>>());
        let num_ancestors = node.ancestors(arena).count();
        let top_node_index = node.ancestors(arena).nth(num_ancestors-1).expect("Could not get top node from arena");
        let top_node = &arena[top_node_index];
        println!("{:?}", top_node.data.name);

        calculate_weights(top_node_index, arena);

    }

}

/*
 * NO SPACE LEFT ON DEVICE
*/

use petgraph::{graph::Graph, EdgeDirection};
use std::{
    fs::File,
    io::{BufReader, Read},
};

struct Node {
    name: String,
    size: usize,
    is_folder: bool,
}

fn parse_commands(input: String) -> Graph<Node, usize> {
    let commands: Vec<&str> = input.trim_end().split("\n").collect();
    println!("{:?}", commands);
    let mut file_graph = Graph::<Node, usize>::new();
    let root = file_graph.add_node(Node {
        name: ("/".to_string()),
        size: 0,
        is_folder: true,
    });
    
    let path_stack = vec![root];
    
    return file_graph;
}
pub fn solution(_input: String) {
    let file = File::options().read(true).open("src/input.txt");
    match file {
        Ok(file) => {
            let mut buff_reader = BufReader::new(file);
            let mut input = String::new();
            let _ = buff_reader.read_to_string(&mut input);

            let file_tree = parse_commands(input);
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}

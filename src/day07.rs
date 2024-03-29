/*
 * NO SPACE LEFT ON DEVICE
*/

use petgraph::{
    graph::Graph,
    visit::{Bfs},
    EdgeDirection,
};

#[derive(Debug)]
struct Node {
    name: String,
    size: usize,
    is_folder: bool,
}

// Credit: https://github.com/gwpmad/advent-of-code-2022/blob/main/src/days/day7.rs
fn parse_commands(input: String) -> Graph<Node, usize> {
    let input_with_sn = format!("{}{}", "\n", input);
    let commands: Vec<&str> = input_with_sn.trim_end().split("\n$ ").collect();

    let mut file_graph = Graph::<Node, usize>::new();
    let root_index = file_graph.add_node(Node {
        name: ("/".to_string()),
        size: 0,
        is_folder: true,
    });
    let mut folder_stack = vec![root_index];

    let file_graph = commands[2..commands.len()]
        .iter()
        .fold(file_graph, |mut file_graph, line| {
            let current_node_index = folder_stack
                .last()
                .expect("No current node index!")
                .to_owned();
            if line.starts_with("cd") {
                let folder = line[3..line.len()].to_string();

                if folder == ".." {
                    folder_stack.pop();
                } else {
                    let new_node_index = file_graph
                        .neighbors_directed(current_node_index, EdgeDirection::Outgoing)
                        .find(|node_index| {
                            file_graph
                                .node_weight(*node_index)
                                .expect("Found node index with no weight")
                                .name
                                == folder
                        })
                        .expect("Couldn't find node index for requested folder {folder}");
                    folder_stack.push(new_node_index);
                }
            }

            if line.starts_with("ls") {
                let split_files = line[3..line.len()].split('\n');
                let mut folder_size: usize = 0;
                for file in split_files {
                    let parts: Vec<&str> = file.split(' ').collect();
                    let entity_name = parts[1].to_string();
                    let is_folder = parts[0] == "dir";

                    let entity_size = if is_folder {
                        0 as usize
                    } else {
                        parts[0]
                            .parse::<usize>()
                            .expect("File size should be a number")
                    };

                    let node_index_for_new_entity = file_graph.add_node(Node {
                        name: entity_name,
                        size: entity_size,
                        is_folder: is_folder,
                    });
                    file_graph.add_edge(current_node_index, node_index_for_new_entity, 1);

                    if !is_folder {
                        folder_size += entity_size;
                    }
                }

                folder_stack.iter().for_each(|node_index| {
                    file_graph
                        .node_weight_mut(*node_index)
                        .expect("Found node index with no weight")
                        .size += folder_size;
                })
            }
            file_graph
        });
    return file_graph;
}

fn part1(input: String) {
    let file_graph = parse_commands(input);
    const MAX_FOLDER_SIZE: usize = 100000;
    let mut bfs = Bfs::new(
        &file_graph,
        file_graph
            .node_indices()
            .nth(0)
            .expect("No 0th index in file_graph"),
    );

    let mut result: usize = 0;
    while let Some(visited) = bfs.next(&file_graph) {
        let entity = file_graph
            .node_weight(visited)
            .expect("Found node index with no weight");

        if entity.is_folder && entity.size <= MAX_FOLDER_SIZE {
            result += entity.size;
        }
    }
    println!("Result: {}", result);
}

fn part2(input: String) {
    let file_graph = parse_commands(input);
    const TOTAL_DISK_SPACE: usize = 70000000;
    const REQUIRED_SPACE: usize = 30000000;
    let available_space = TOTAL_DISK_SPACE
        - file_graph
            .node_weight(
                file_graph
                    .node_indices()
                    .nth(0)
                    .expect("No 0th index in file_graph"),
            )
            .expect("Couldn't find first node")
            .size;
    let minimum_size_to_delete = REQUIRED_SPACE - available_space;

    let mut bfs = Bfs::new(
        &file_graph,
        file_graph
            .node_indices()
            .nth(0)
            .expect("No 0th index in file_graph"),
    );

    let mut result: usize = TOTAL_DISK_SPACE;
    while let Some(visited) = bfs.next(&file_graph) {
        let entity = file_graph
            .node_weight(visited)
            .expect("Found node index with no weight");

        if entity.is_folder && entity.size >= minimum_size_to_delete && entity.size < result {
            result = entity.size
        }
    }

    println!("Result: {}", result);
}

pub fn solution(input: String) {
    part1(input.clone());
    part2(input.clone());
}

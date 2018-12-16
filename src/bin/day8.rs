use aoc;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let arg = aoc::get_cmdline_arg()?;

    let numbers = BufReader::new(File::open(arg)?)
        .lines()
        .next()
        .unwrap()?
        .split(" ")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    part1(&numbers);
    part2(&numbers);

    Ok(())
}

fn part1(numbers: &[usize]) {
    let mut sum = 0;

    sum_metadatas(numbers, 0, &mut sum);

    println!("part 1: {}", sum);
}

fn sum_metadatas(numbers: &[usize], index: usize, sum: &mut usize) -> usize {
    let child_nodes = numbers[index];
    let metadata_count = numbers[index + 1];

    let next_index = if child_nodes == 0 {
        index + 2 + metadata_count
    } else {
        let mut next_index = sum_metadatas(numbers, index + 2, sum);
        for _ in 1..child_nodes {
            next_index = sum_metadatas(numbers, next_index, sum);
        }

        next_index + metadata_count
    };

    for i in 0..metadata_count {
        *sum += numbers[next_index - i - 1];
    }
    next_index
}

fn part2(numbers: &[usize]) {
    let mut nodes = HashMap::new();
    let mut ids = Vec::new();
    for i in 0..numbers.len() {
        ids.push(i + 1);
    }

    get_nodes(numbers, 0, &mut ids, &mut nodes);
    let node_value = get_node_value(&nodes, numbers.len());
    println!("part 2: {}", node_value);
}

#[derive(Debug)]
enum NodeType {
    NoChildren(usize),
    // child ids, metadata entries
    Children((Vec<usize>, Vec<usize>)),
}

fn get_nodes(
    numbers: &[usize],
    index: usize,
    ids: &mut Vec<usize>,
    nodes: &mut HashMap<usize, NodeType>,
) -> (usize, usize) {
    let child_nodes = numbers[index];
    let metadata_count = numbers[index + 1];

    let node_id = ids.pop().unwrap();
    let next_index = if child_nodes == 0 {
        let next_index = index + 2 + metadata_count;
        let mut sum = 0;
        for i in 0..metadata_count {
            sum += numbers[next_index - i - 1];
        }
        nodes.insert(node_id, NodeType::NoChildren(sum));
        next_index
    } else {
        let (mut next_index, child_id) = get_nodes(numbers, index + 2, ids, nodes);
        let mut child_ids = vec![child_id];
        for _ in 1..child_nodes {
            let (next, child_id) = get_nodes(numbers, next_index, ids, nodes);
            child_ids.push(child_id);
            next_index = next;
        }

        let next_index = next_index + metadata_count;
        for i in 0..metadata_count {
            let metadata = numbers[next_index - i - 1];
            match nodes.entry(node_id) {
                Entry::Occupied(mut o) => match o.get_mut() {
                    NodeType::Children(v) => v.1.push(metadata),
                    _ => {}
                },
                Entry::Vacant(v) => {
                    v.insert(NodeType::Children((child_ids.clone(), vec![metadata])));
                }
            }
        }

        next_index
    };

    (next_index, node_id)
}

fn get_node_value(nodes: &HashMap<usize, NodeType>, node: usize) -> usize {
    let mut sum = 0;

    if let Some(n) = nodes.get(&node) {
        match n {
            NodeType::NoChildren(s) => {
                sum += s;
            }
            NodeType::Children((child_nodes, v)) => {
                for i in v {
                    if *i > 0 {
                        let index = i - 1;
                        if index < child_nodes.len() {
                            sum += get_node_value(nodes, child_nodes[index]);
                        }
                    }
                }
            }
        }
    }

    sum
}

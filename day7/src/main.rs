use std::collections::{HashMap, BinaryHeap};
use std::error::Error;
use std::io;
use std::io::Read;
use std::str::FromStr;

use scan_fmt::scan_fmt;

type Result<T> = std::result::Result<T, Box<Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let mut deps: Vec<Dep> = Vec::new();

    for line in input.lines() {
        deps.push(line.parse()?);
    }

    let graph = Graph::new(&deps);
    part1(&graph);
    Ok(())
}

fn part1(graph: &Graph) {
    /* map of node => number of dependencies */
    let mut dep_map = HashMap::new();
    /* priority queue for BFS */
    let mut queue: BinaryHeap<&Node> = BinaryHeap::new();

    for node in graph.map.values() {
        if node.dep_nodes.len() == 0 {
            /* add sources in queue */
            queue.push(node);
        } else {
            dep_map.insert(node.name, node.dep_nodes.len());
        }
    }

    let mut res = String::new();
    while let Some(node) = queue.pop() {
        res.push(node.name);

        /* push all new next nodes in queue */
        for next in node.next_nodes.iter() {
            let next_node = graph.map.get(&next).unwrap();
            let nb_deps = dep_map.get_mut(&next_node.name).unwrap();

            if *nb_deps == 1 {
                queue.push(next_node);
            } else {
                *nb_deps -= 1;
            }
        }
    }

    println!("day7, part1: sequence is {}", res);
}

#[derive(Eq, PartialEq, Debug)]
struct Node {
    name: char,
    next_nodes: Vec<char>,
    dep_nodes: Vec<char>,
}

/* sort by name increasingly, for the priority queue */
impl Ord for Node {
    fn cmp(&self, other: &Node) -> std::cmp::Ordering {
        other.name.cmp(&self.name)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Graph {
    map: HashMap<char, Node>,
}

impl Graph {
    fn get_or_add_node(&mut self, name: char) -> &mut Node {
        self.map.entry(name).or_insert(Node {
            name,
            next_nodes: Vec::new(),
            dep_nodes: Vec::new(),
        })
    }

    fn new(deps: &Vec<Dep>) -> Self {
        let mut graph = Graph {
            map: HashMap::new(),
        };

        for dep in deps {
            let node = graph.get_or_add_node(dep.dep);
            node.next_nodes.push(dep.step);

            let node = graph.get_or_add_node(dep.step);
            node.dep_nodes.push(dep.dep);
        }
        graph
    }
}

struct Dep {
    step: char,
    dep: char,
}

impl FromStr for Dep {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self> {
        let (dep, step) = scan_fmt!(
            s,
            "Step {} must be finished before step {} can begin.",
            char,
            char
        );
        Ok(Dep {
            step: step.unwrap(),
            dep: dep.unwrap(),
        })
    }
}

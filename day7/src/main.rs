use std::collections::{BinaryHeap, HashMap};
use std::error::Error;
use std::io;
use std::io::Read;
use std::iter::Iterator;
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
    part2(&graph);
    Ok(())
}

fn part1(graph: &Graph) {
    let mut res = String::new();
    let mut walker = GraphWalker::new(&graph);

    while let Some(node) = walker.queue.pop() {
        res.push(node.name);
        walker.add_next_nodes(node);
    }

    println!("day7, part1: sequence is {}", res);
}

fn part2(graph: &Graph) {
    let mut workers = WorkersPool::new(5);
    let mut processing_nodes: BinaryHeap<ProcessingEnd> = BinaryHeap::new();
    let mut walker = GraphWalker::new(&graph);
    let mut current_time = 0;

    while walker.queue.len() > 0 || processing_nodes.len() > 0 {
        /* advance in time to the next available worker */
        let worker = workers.get_next_available_worker();
        current_time = std::cmp::max(worker.available_at, current_time);

        /* add next nodes for all nodes processed */
        /* FIXME: there is probably a better way to pop conditionally */
        while processing_nodes.len() > 0 {
            if processing_nodes.peek().unwrap().finished_on <= current_time {
                let p = processing_nodes.pop().unwrap();
                walker.add_next_nodes(p.node);
            } else {
                break;
            }
        }

        /* consume node available at that time */
        match walker.queue.pop() {
            Some(node) => {
                let finished_on = current_time + node.processing_cost() + 60;
                worker.available_at = finished_on;

                processing_nodes.push(ProcessingEnd { node, finished_on });
            },
            None => {
                /* no available node, advance until next processed node */
                if let Some(p) = processing_nodes.peek() {
                    current_time = p.finished_on;
                }
            }
        }
    }

    /* the final time is when the last worker is done */
    println!("day7, part2: total time is {}", workers.get_final_time());
}

/* {{{ WorkersPool */

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Worker {
    /* indicate the next time the worker is available */
    available_at: u32,
}

struct WorkersPool {
    workers: Vec<Worker>,
}

impl WorkersPool {
    fn new(nb_workers: u32) -> Self {
        WorkersPool {
            workers: vec![Worker { available_at: 0 }; nb_workers as usize],
        }
    }

    fn get_next_available_worker(&mut self) -> &mut Worker {
        self.workers.iter_mut().min().unwrap()
    }

    fn get_final_time(&self) -> u32 {
        self.workers.iter().max().unwrap().available_at
    }
}

/* }}} */
/* {{{ ProcessingEnd */

#[derive(Eq, PartialEq, Debug)]
struct ProcessingEnd<'a> {
    node: &'a Node,
    finished_on: u32,
}

/* sort by name increasingly, for the priority queue */
impl<'a> Ord for ProcessingEnd<'a> {
    fn cmp(&self, other: &ProcessingEnd<'a>) -> std::cmp::Ordering {
        other.finished_on.cmp(&self.finished_on)
    }
}

impl<'a> PartialOrd for ProcessingEnd<'a> {
    fn partial_cmp(&self, other: &ProcessingEnd<'a>) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/* }}} */
/* {{{ Node */

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

impl Node {
    fn processing_cost(&self) -> u32 {
        (self.name as u32) - ('A' as u32) + 1
    }
}

/* }}} */
/* {{{ Graph */

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

struct GraphWalker<'a> {
    graph: &'a Graph,
    dep_map: HashMap<char, usize>,
    queue: BinaryHeap<&'a Node>,
}

impl<'a> GraphWalker<'a> {
    fn new(graph: &'a Graph) -> Self {
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

        GraphWalker {
            graph,
            dep_map,
            queue,
        }
    }

    fn add_next_nodes(&mut self, node: &Node) {
        /* push all new next nodes in queue */
        for next in node.next_nodes.iter() {
            let next_node = self.graph.map.get(&next).unwrap();
            let nb_deps = self.dep_map.get_mut(&next_node.name).unwrap();

            if *nb_deps == 1 {
                self.queue.push(next_node);
            } else {
                *nb_deps -= 1;
            }
        }
    }
}

/* }}} */
/* {{{ Dep */

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

/* }}} */

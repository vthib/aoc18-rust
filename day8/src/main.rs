use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::io::Read;
use std::iter::Iterator;

type Result<T> = std::result::Result<T, Box<Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let line = input.lines().into_iter().next().unwrap();
    let mut iter = line.split(' ').map(|s| s.parse::<u32>().unwrap());
    let root = Node::new(&mut iter);

    println!("day8, part1: total is {}", part1(&root));
    println!("day8, part2: value is {}", root.value());
    Ok(())
}

fn part1(node: &Node) -> u32 {
    let children_sum: u32 = node.children.iter().map(|n| part1(n)).sum();
    let node_sum: u32 = node.metadatas.iter().sum();

    children_sum + node_sum
}

/* {{{ Node */

struct Node {
    children: Vec<Node>,
    metadatas: Vec<u32>,
}

impl Node {
    fn new<I>(input: &mut I) -> Self
        where I: Iterator<Item = u32>
    {
        let nb_children = input.next().unwrap();
        let nb_meta = input.next().unwrap();

        let mut children = Vec::with_capacity(nb_children as usize);
        for _ in 0..nb_children {
            children.push(Node::new(input));
        }

        let mut metadatas = Vec::with_capacity(nb_meta as usize);
        for _ in 0..nb_meta {
            metadatas.push(input.next().unwrap());
        }

        Node {
            children,
            metadatas,
        }
    }

    fn value(&self) -> u32 {
        if self.children.len() == 0 {
            self.metadatas.iter().sum()
        } else {
            let mut cache = HashMap::new();
            let mut sum = 0;

            for meta in self.metadatas.iter() {
                let index = *meta - 1;
                if let Some(child) = self.children.get(index as usize) {
                    let value = match cache.get(&index) {
                        Some(value) => *value,
                        None => {
                            let value = child.value();
                            cache.insert(index, value);
                            value
                        }
                    };
                    sum += value;
                }
            };
            sum
        }
    }
}

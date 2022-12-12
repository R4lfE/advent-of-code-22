use std::{error::Error, fs, collections::HashMap};

type Value = Option<u32>;

#[allow(dead_code)]
#[derive(Debug)]
struct Node {
    idx: usize,
    key: String,
    val: Value,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl Node {
    fn is_file(&self) -> bool {
        self.children.is_empty()
    }
}

#[derive(Debug)]
struct Tree {
    tree: Vec<Node>,
    key_map: HashMap<String, (usize, Value)>,
}

impl Tree {
    fn new() -> Tree {
        Tree {
            tree: vec![],
            key_map: HashMap::new(),
        }
    }

    fn add_node(&mut self, key: String, val: Value, parent: Option<usize>) {
        let idx = self.tree.len();
        if let Some(res) = self.key_map.insert(key.clone(), (idx, val)) {
            panic!("Entry overwritten on key '{}'. Found {:?}.", key, res);
        }

        if let Some(p) = parent {
            self.tree[p].children.push(idx);
        }

        self.tree.push(Node{
            idx,
            key,
            val,
            parent,
            children: vec![],
        });
    }

    fn calc_vals(&mut self, node: usize) -> u32 {
        // return if value present
        if let Some(val) = self.tree[node].val {
            val
        } else {
            // cloned children pointer vector
            let children = self.tree[node].children.clone();

            // sum children values
            let val = children
                .iter()
                .fold(0, |acc, child| {
                    acc + self.calc_vals(*child)
                });
            
            // set and return value
            self.tree[node].val = Some(val);
            self.key_map.insert(self.tree[node].key.clone(), (node, Some(val)));
            val
        }
    }

    fn find_sum_at_most_x(&self, node: usize, acc: &mut u32, max_size: u32) {
        let val = self.tree[node].val.expect(format!("No value found on node {}", node).as_str());
        
        if !self.tree[node].is_file() && val <= max_size {
            *acc += val;
        }

        // cloned children pointer vector
        let children = self.tree[node].children.clone();

        // accumulate values
        children
            .iter()
            .for_each(|child| self.find_sum_at_most_x(*child, acc, max_size));
    }

    fn find_best_file(&self, disk_space: u32, update_size: u32) -> u32 {
        let free_space = disk_space - self.tree[0].val.unwrap();
        let mut best = u32::MAX;
        for node in self.tree.iter() {
            let Some(val) = node.val else {
                panic!("Node doesn't have a value. {:#?}", node);
            };
            
            if val >= update_size - free_space && val < best {
                best = val;
            }
        }
        best
    } 
}

fn parse_tree(input: String) -> Tree {
    // tree structure
    let mut tree = Tree::new();

    // add root directory
    tree.add_node(String::from("/"), None, None);
    
    // current directory pointer
    let mut current = 0;

    // read directories
    let input = input.lines().skip(1).collect::<Vec<&str>>();
    let mut iter = input.iter().peekable();

    let mut idx = 1;
    while let Some(&l) = iter.next() {
        // outer while is always a '$'
        let split = l.split_ascii_whitespace().collect::<Vec<&str>>();

        // match the command
        match split[1] {
            "cd" => {
                match split[2] {
                    "/" => current = 0,

                    // go up a level
                    ".." => current = tree.tree[current].parent.unwrap(),

                    // go down a level
                    key => current = *tree.tree[current].children
                        .iter()
                        .filter(|&child| tree.tree[*child].key.contains(key))
                        .collect::<Vec<&usize>>()[0],
                }
            },
            "ls" => {
                // peek to not miss the next command
                while let Some(&&ls) = iter.peek() {
                    let split  = ls.split_ascii_whitespace().collect::<Vec<&str>>();
                    match split[0] {
                        "dir" => {
                            tree.add_node(split[1].to_string() + &idx.to_string(), None, Some(current));
                            idx += 1;
                            iter.next();
                        },
                        val if val.parse::<u32>().is_ok() => {
                            tree.add_node(split[1].to_string() + &idx.to_string(), Some(val.parse::<u32>().unwrap()), Some(current));
                            idx += 1;
                            iter.next();
                        },
                        _ => break,
                    }
                }
            },
            _ => panic!("Invalid argument."),
        }
        
        idx += 1;
    }

    tree
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let mut tree = parse_tree(input);
    tree.calc_vals(0);

    let mut part_1 = 0;
    tree.find_sum_at_most_x(tree.key_map["/"].0, &mut part_1, 100_000);
    println!("{}", part_1);

    let part_2 = tree.find_best_file(70_000_000, 30_000_000);
    println!("{}", part_2);

    Ok(())
}

use std::{
    cell::RefCell,
    char,
    collections::HashMap,
    fmt::Debug,
    io::{self, BufRead},
    rc::{Rc, Weak},
};

#[derive(Debug)]
enum NodeType {
    Directory,
    File,
}

type NodePtr = Rc<RefCell<Node>>;
type NodeWeakPtr = Weak<RefCell<Node>>;

struct Node {
    name: String,
    node_type: NodeType,
    size: usize,
    children: HashMap<String, NodePtr>,
    parent: NodeWeakPtr,
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Node {{ name = {}, type = {:?}, size = {}, children = {:?}, parent = {:?} }}",
            self.name,
            self.node_type,
            self.size,
            self.children.keys().collect::<Vec<&String>>(),
            self.parent.upgrade()
        )
    }
}

fn parse_input(lines: &[String]) -> NodePtr {
    let root = Rc::new(RefCell::new(Node {
        name: "/".to_string(),
        node_type: NodeType::Directory,
        size: 0,
        children: HashMap::new(),
        parent: Weak::new(),
    }));
    let mut cwd = Rc::clone(&root);
    let mut dirstack: Vec<NodePtr> = vec![];
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[..] {
            ["$", "cd", "/"] => {}
            ["$", "cd", ".."] => {
                let tmp = cwd.borrow().parent.upgrade();
                if tmp.is_none() {
                    continue;
                }
                cwd = tmp.unwrap();
                dirstack.pop();
            }
            ["$", "cd", name] => {
                dirstack.push(Rc::clone(&cwd));
                let tmp = cwd.borrow().children.get(name).unwrap().clone();
                cwd = tmp;
            }
            ["$", "ls"] => {}
            ["dir", name] => {
                let new_node = Node {
                    name: name.to_string(),
                    node_type: NodeType::Directory,
                    size: 0,
                    children: HashMap::new(),
                    parent: Rc::downgrade(&cwd),
                };
                cwd.borrow_mut()
                    .children
                    .insert(name.to_string(), Rc::new(RefCell::new(new_node)));
            }
            [filesize, name] if filesize.starts_with(|c: char| c.is_ascii_digit()) => {
                cwd.borrow_mut().children.insert(
                    name.to_string(),
                    Rc::new(RefCell::new(Node {
                        name: name.to_string(),
                        node_type: NodeType::File,
                        size: filesize.parse().unwrap(),
                        children: HashMap::new(),
                        parent: Rc::downgrade(&cwd),
                    })),
                );
            }
            _ => {}
        }
    }
    root
}

fn sum_up(node: NodePtr) -> usize {
    let mut node_size;
    {
        let n = node.borrow();
        node_size = n.size;
        for child in n.children.values() {
            node_size += sum_up(Rc::clone(child));
        }
    }

    {
        let mut n = node.borrow_mut();
        n.size = node_size;
    }

    node_size
}

fn free_up(node: NodePtr, free_space: usize, candidates: &mut Vec<usize>) {
    let n = node.borrow();
    match n.node_type {
        NodeType::Directory => {
            if free_space + n.size >= 30_000_000 {
                candidates.push(n.size);
            }
        }
        NodeType::File => {}
    }
    for child in n.children.values() {
        free_up(child.clone(), free_space, candidates);
    }
}

fn solution(node: NodePtr) -> usize {
    sum_up(node.clone());
    let free_space = 70_000_000 - node.borrow().size;
    let mut candidates: Vec<usize> = vec![];
    free_up(node, free_space, &mut candidates);
    candidates.into_iter().min().unwrap()
}

fn main() {
    let lines = io::stdin().lock().lines();
    let lines: Vec<String> = lines.map(|line| line.unwrap()).collect();
    println!("{}", solution(parse_input(&lines)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let lines = [
            "$ cd /",
            "$ ls",
            "dir a",
            "14848514 b.txt",
            "8504156 c.dat",
            "dir d",
            "$ cd a",
            "$ ls",
            "dir e",
            "29116 f",
            "2557 g",
            "62596 h.lst",
            "$ cd e",
            "$ ls",
            "584 i",
            "$ cd ..",
            "$ cd ..",
            "$ cd d",
            "$ ls",
            "4060174 j",
            "8033020 d.log",
            "5626152 d.ext",
            "7214296 k",
        ];

        let lines: Vec<String> = lines.iter().map(|x| x.to_string()).collect();
        let node = parse_input(&lines);
        assert_eq!(solution(node), 24933642);
    }
}

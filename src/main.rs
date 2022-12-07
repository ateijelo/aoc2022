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
        // println!("cwd = {:?}", cwd);
        match parts[..] {
            ["$", "cd", "/"] => {
                // println!("cd'ing to root dir");
            }
            ["$", "cd", ".."] => {
                // println!("cd'ing to parent =============");
                // println!("==== cwd: {:?}", cwd);
                let tmp = cwd.borrow().parent.upgrade();
                // println!("==== tmp: {:?}", tmp);
                if tmp.is_none() {
                    continue;
                }
                cwd = tmp.unwrap();
                dirstack.pop();
                // println!("cwd = {:?}", cwd);
            }
            ["$", "cd", name] => {
                // println!("before cd'ing to {} cwd = {:?}", name, cwd);
                // keep old cwd's alive so cd .. works
                dirstack.push(Rc::clone(&cwd));
                let tmp = cwd.borrow().children.get(name).unwrap().clone();
                cwd = tmp;
                // println!("after cd'ing to {} cwd = {:?}", name, cwd);
            }
            ["$", "ls"] => {
                // println!("doing ls");
            }
            ["dir", name] => {
                // println!("found dir {}", name);
                // println!("==== cwd: {:?}", cwd);
                let new_node = Node {
                    name: name.to_string(),
                    node_type: NodeType::Directory,
                    size: 0,
                    children: HashMap::new(),
                    parent: Rc::downgrade(&cwd),
                };
                // println!("new_node: {:?}", new_node);
                cwd.borrow_mut()
                    .children
                    .insert(name.to_string(), Rc::new(RefCell::new(new_node)));
                // println!("cwd: {:?}", cwd);
            }
            [filesize, name] if filesize.starts_with(|c: char| c.is_ascii_digit()) => {
                // println!("found file named {} with size {}", name, filesize);
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
            _ => {
                println!("something else");
            }
        }
    }
    root
}

fn walk(node: NodePtr, indent: usize, result: &mut usize) -> usize {
    let n = node.borrow();

    let mut node_size = n.size;
    for child in n.children.values() {
        node_size += walk(Rc::clone(child), indent + 2, result);
    }

    match n.node_type {
        NodeType::Directory => {
            println!("{}🗁 {} ({})", " ".repeat(indent), n.name, node_size);
            if node_size <= 100000 {
                *result += node_size;
            }
        }
        NodeType::File => {
            println!("{}🗋 {} ({})", " ".repeat(indent), n.name, node_size);
        }
    }
    node_size
}

fn solution(node: NodePtr) -> usize {
    let mut result = 0;
    walk(node, 0, &mut result);
    result
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
        assert_eq!(solution(node), 95437);
     }
}

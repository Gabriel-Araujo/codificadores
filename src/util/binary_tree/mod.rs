use std::collections::VecDeque;
use std::rc::Rc;

use super::token::Token;

#[derive(Debug, Clone)]
pub struct Node {
    parent: Option<Box<Node>>,
    left: Option<Box<Node>>,  // 0
    right: Option<Box<Node>>, // 1
    symbol: Option<char>,
    count: f64,
}

impl Node {
    pub fn new(symbol: char, count: f64) -> Self {
        Node {
            parent: None,
            left: None,
            right: None,
            symbol: Some(symbol),
            count,
        }
    }

    pub fn new_branch(left: Node, right: Node) -> Self {
        Node {
            count: left.count + right.count,
            parent: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
            symbol: None,
        }
    }

    pub fn set_parent(&mut self, parent: Node) {
        self.parent = Some(Box::new(parent))
    }

    pub fn get_count(&self) -> f64 {
        self.count
    }

    pub fn get_symbol(&self) -> char {
        if self.symbol.is_none() {
            return ' ';
        }
        self.symbol.unwrap()
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

pub struct BinaryTree {
    root: Rc<Node>,
}

impl BinaryTree {
    // deve retornar em caso de sucesso o código
    pub fn new(table: Vec<Token>) -> Self {
        let mut list: VecDeque<Node> = table
            .iter()
            .map(|f| Node::new(f.get_symbol(), f.get_probability()))
            .collect();

        while list.len() > 1 {
            let left = list.pop_front();
            let right = list.pop_front();

            let branch = Node::new_branch(left.unwrap(), right.unwrap());
            let index = list.partition_point(|f| f.get_count() <= branch.get_count());

            list.insert(index, branch);
        }

        let root = list.get(0).unwrap();

        BinaryTree {
            root: Rc::new(root.to_owned()),
        }
    }

    pub fn get_root(&self) -> &Node {
        self.root.as_ref()
    }

    // retorna um número de 8 bits em que sua forma binária serve como código
    // Implementa um Depth First Search
    pub fn search(root: &Node, value: char) -> Option<u8> {
        BinaryTree::bfs_search(root, value, 0)
    }

    fn bfs_search(root: &Node, value: char, code: u8) -> Option<u8> {
        let symbol = root.symbol;

        if symbol.is_some() {
            return if symbol.unwrap() == value {
                Some(code)
            } else {
                None
            };
        }

        let mut left_branch = None;
        if root.left.is_some() {
            left_branch = BinaryTree::bfs_search(root.left.as_ref().unwrap(), value, code << 1);
        };

        let mut right_branch = None;
        if root.right.is_some() {
            right_branch =
                BinaryTree::bfs_search(root.right.as_ref().unwrap(), value, (code << 1) + 1);
        }

        if left_branch.is_some() {
            left_branch
        } else {
            right_branch
        }
    }

    pub fn traversal(&self) {
        let mut queue: VecDeque<&Node> = VecDeque::new();
        queue.push_front(self.get_root());

        while !queue.is_empty() {
            let first = queue.pop_back().unwrap();

            if first.is_leaf() {
                println!("{} - {}", first.symbol.unwrap(), first.count);
            }

            match &first.left {
                None => {}
                Some(t) => queue.push_front(t),
            }
            match &first.right {
                None => {}
                Some(t) => queue.push_front(t),
            }
        }
    }
}

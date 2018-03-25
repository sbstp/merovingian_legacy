use slab::Slab;

struct Links<T> {
    parent: Option<Node>,
    children: Vec<Node>,
    data: T,
}

impl<T> Links<T> {
    fn empty(data: T) -> Links<T> {
        Links {
            parent: None,
            children: vec![],
            data,
        }
    }
}

pub struct Tree<T> {
    nodes: Slab<Links<T>>,
}

impl<T> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree { nodes: Slab::new() }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn node(&mut self, data: T) -> Node {
        let id = self.nodes.insert(Links::empty(data));
        Node(id)
    }

    pub fn append_to(&mut self, child: Node, parent: Node) {
        // if the child node already has a parent, remove it from the parent
        if let Some(parent) = self.parent(child) {
            self.remove_from(child, parent);
        }
        self.nodes[parent.0].children.push(child);
        self.nodes[child.0].parent = Some(parent);
    }

    pub fn remove_from(&mut self, child: Node, parent: Node) {
        self.nodes[parent.0].children.retain(|n| n.0 != child.0);
        self.nodes[child.0].parent = None;
    }

    #[inline]
    pub fn data(&self, node: Node) -> &T {
        &self.nodes[node.0].data
    }

    #[inline]
    pub fn data_mut(&mut self, node: Node) -> &mut T {
        &mut self.nodes[node.0].data
    }

    #[inline]
    pub fn parent(&self, node: Node) -> Option<Node> {
        self.nodes[node.0].parent
    }

    pub fn children(&self, node: Node) -> Vec<Node> {
        self.nodes[node.0].children.iter().cloned().collect()
    }

    pub fn siblings(&self, node: Node) -> Vec<Node> {
        match self.nodes[node.0].parent {
            None => vec![],
            Some(parent) => self.nodes[parent.0]
                .children
                .iter()
                .filter(|n| n.0 != node.0)
                .cloned()
                .collect(),
        }
    }

    pub fn recursive_iter(&self, node: Node) -> RecursiveIter<T> {
        RecursiveIter {
            tree: self,
            nodes: vec![node],
        }
    }
}

pub struct RecursiveIter<'t, T: 't> {
    tree: &'t Tree<T>,
    nodes: Vec<Node>,
}

impl<'t, T> Iterator for RecursiveIter<'t, T> {
    type Item = Node;

    fn next(&mut self) -> Option<Node> {
        if let Some(node) = self.nodes.pop() {
            self.nodes.extend(self.tree.children(node));
            return Some(node);
        }

        None
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Node(usize);

#[test]
fn test_create_append() {
    let mut tree = Tree::new();
    let root = tree.node("hello");
    let child1 = tree.node("abc");
    let child2 = tree.node("def");

    tree.append_to(child1, root);
    tree.append_to(child2, root);

    assert_eq!(tree.children(root), vec![child1, child2]);
}

#[test]
fn test_remove() {
    let mut tree = Tree::new();
    let root = tree.node("hello");
    let child1 = tree.node("abc");
    let child2 = tree.node("def");

    tree.append_to(child1, root);
    tree.append_to(child2, root);

    tree.remove_from(child1, root);
    assert_eq!(tree.children(root), vec![child2]);
}

#[test]
fn test_append_remove() {
    let mut tree = Tree::new();
    let root1 = tree.node("hello1");
    let root2 = tree.node("hello2");
    let child1 = tree.node("abc");
    let child2 = tree.node("def");

    tree.append_to(child1, root1);
    tree.append_to(child2, root1);

    tree.append_to(child1, root2);

    assert_eq!(tree.children(root1), vec![child2]);
}

#[test]
fn test_parent() {
    let mut tree = Tree::new();
    let root = tree.node("hello");
    let child1 = tree.node("abc");

    tree.append_to(child1, root);

    assert_eq!(tree.parent(child1), Some(root));
    tree.remove_from(child1, root);
    assert_eq!(tree.parent(child1), None);
}

#[test]
fn test_siblings() {
    let mut tree = Tree::new();
    let root = tree.node("hello");
    let child1 = tree.node("abc");
    let child2 = tree.node("def");
    let child3 = tree.node("hij");

    tree.append_to(child1, root);
    tree.append_to(child2, root);
    tree.append_to(child3, root);

    assert_eq!(tree.siblings(child1), vec![child2, child3]);
}

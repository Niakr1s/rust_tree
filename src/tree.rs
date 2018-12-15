use std::fmt;

#[derive(Debug)]
pub struct Forest<'a> {
    trees: Vec<Tree<'a>>,
}

impl<'a> Forest<'a> {
    pub fn new() -> Forest<'a> {
        let mut root = Tree::new("root");
        root.id = Some(NodeID { index: 0 });
        root.depth = Some(0);
        let trees = vec![root];
        Forest { trees }
    }
    pub fn new_node(&mut self, name: &'a str) -> NodeID {
        let index = self.trees.len();

        let mut node = Tree::new(name);
        let nodeid = NodeID { index };
        node.id = Some(nodeid.clone());
        self.push_node(node);

        nodeid
    }
    pub fn push_node(&mut self, node: Tree<'a>) {
        self.trees.push(node);
    }
    pub fn get_root(&self) -> NodeID {
        let selfID = &self.trees[0];
        if let Some(ref id) = selfID.id {
            NodeID { index: id.index }
        } else {
            NodeID { index: 0 }
        }
    }
    pub fn get_mut(&mut self, nodeid: &NodeID) -> &mut Tree<'a> {
        &mut self.trees[nodeid.index]
    }
    pub fn get(&self, nodeid: &NodeID) -> &Tree<'a> {
        &self.trees[nodeid.index]
    }
}

impl<'a> fmt::Display for Forest<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for node in &self.trees {
            let depth = node.depth.unwrap();
            let tab = " ";
            for i in 0..depth {
                write!(f, "{}", tab);
            }
            writeln!(f, "#{}", node.name);
        }
        write!(f, "")
    }
}

#[derive(Debug)]
struct Tree<'a> {
    name: &'a str,
    depth: Option<usize>,
    id: Option<NodeID>,
    parent: Option<NodeID>,
    children: Vec<NodeID>,
}

impl<'a> Tree<'a> {
    fn new(name: &'a str) -> Tree<'a> {
        Tree {
            name,
            id: None,
            depth: None,
            parent: None,
            children: Vec::new(),
        }
    }
    fn add_child(&mut self, nodeid: &NodeID) {
        self.children.push(nodeid.clone())
    }
}

#[derive(Debug, Clone)]
pub struct NodeID {
    index: usize,
}

impl<'a> NodeID {
    pub fn append(&self, name: &'a str, f: &mut Forest<'a>) -> NodeID {
        let mut child_id = f.new_node(name);
        {
            let mut parent = f.get_mut(&self);
            parent.add_child(&child_id);
        }

        let mut parent_depth: Option<usize> = None;
        {
            parent_depth = self.get_depth(f);
        }
        let mut child = f.get_mut(&child_id);
        child.parent = Some(self.clone());
        child.depth = Some(parent_depth.unwrap_or(0) + 1);
        child_id
    }
    fn get_depth(&self, f: &Forest<'a>) -> Option<usize> {
        let node = f.get(&self);
        node.depth
    }
}

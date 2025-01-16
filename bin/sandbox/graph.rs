use std::collections::BTreeSet;
use std::rc::Rc;
use std::cell::RefCell;
use hexmacros::prototype_macro;

pub struct Node<T> {
    pub value: T,
    connections: BTreeSet<NodeId>,
    connected: BTreeSet<NodeId>,
}

impl<T> std::ops::Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> std::ops::DerefMut for Node<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> Node<T> {
    pub const fn new(value: T) -> Self {
        Self {
            value,
            connections: BTreeSet::new(),
            connected: BTreeSet::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConnectionId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeConnection(NodeId, ConnectionId);

pub struct Graph<T> {
    nodes: Vec<Node<T>>,
}

impl<T> Graph<T> {
    
}

prototype_macro!{
    let mut graph = Graph::<&'static str>::new();
    // root -> a -> b -> c -> i -> j
    //     \-> d -> e -> h -/
    //     |-> f -> g -/
    let j = graph.add("j");
    let i = graph.add_with("i", [j]);
    let c = graph.add_with("c", [i]);
    let b = graph.add_with("b", [c]);
    let a = graph.add_with("a", [b]);
    let h = graph.add_with("h", [i]);
    let e = graph.add_with("e", [h]);
    let d = graph.add_with("d", [e]);
    let g = graph.add_with("g", [h]);
    let f = graph.add_with("f", [g]);
    let root = graph.add_with("root", [a, d, f]);
    
}

/* Graph Node List Requirements:
- O(1) insertion
- O(1) deletion
- O(n) iteration
- unordered
*/

pub struct NodeList {
    
}
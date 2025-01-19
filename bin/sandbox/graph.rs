use std::collections::BTreeSet;
use std::hash::Hash;
use std::rc::Rc;
use std::cell::RefCell;
use hexmacros::prototype;
use hexahedron::util::lowlevel::UnsafeArray;
use hexahedron::prelude::Replace;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct SlotId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(u32);

impl SlotId {
    fn node_id(self) -> NodeId {
        NodeId(self.0)
    }
}

impl NodeId {
    fn slot_id(self) -> SlotId {
        SlotId(self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Connection {
    node: SlotId,
    port: SlotId,
}

impl Connection {
    const fn new(node: SlotId, port: SlotId) -> Self {
        Self { node, port }
    }
}

pub struct Node<T> {
    pub value: T,
    outputs: UnorderedVec<Connection>,
    inputs: UnorderedVec<Connection>,
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
            outputs: UnorderedVec::new(),
            inputs: UnorderedVec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConnectionId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeConnection(NodeId, ConnectionId);

pub struct Graph<T> {
    nodes: UnorderedVec<Node<T>>,
}

impl<T> Graph<T> {
    pub const fn new() -> Self {
        Self { nodes: UnorderedVec::new() }
    }

    pub fn insert(&mut self, value: T) -> NodeId {
        self.insert_with(value, [])
    }

    pub fn insert_with<It: IntoIterator<Item = NodeId>>(&mut self, value: T, connections: It) -> NodeId {
        let id = self.nodes.insert(Node::new(value));

        for connection in connections {
            self.connect(id.node_id(), connection);
        }

        id.node_id()
    }

    pub fn connect(&mut self, a: NodeId, b: NodeId) -> Connection {
        let a = a.slot_id();
        let b = b.slot_id();
        let a_connection_slot = self.nodes[a].outputs.insert(Connection::new(b, SlotId(0)));
        let a_connection = Connection::new(a, a_connection_slot);
        let b_connection_slot = self.nodes[b].inputs.insert(a_connection);
        self.nodes[a].outputs[a_connection_slot].port = b_connection_slot;
        a_connection
    }

    pub fn disconnect(&mut self, connection: Connection) {
        let a = connection.node;
        let b_port = connection.port;
        let b_connection = self.nodes[a].outputs[b_port];
        self.nodes[b_connection.node].inputs.remove(b_connection.port);
        self.nodes[a].outputs.remove(b_port);
    }

    pub fn two_way_connect(&mut self, a: NodeId, b: NodeId) -> (Connection, Connection) {
        (
            self.connect(a, b),
            self.connect(b, a),
        )
    }

    pub fn check_connected(&mut self, a: NodeId, b: NodeId, check_inputs: bool) -> bool {
        let a = a.slot_id();
        for output in self.nodes[a].outputs.iter() {
            if output.node == b.slot_id() {
                return true;
            }
        }
        if !check_inputs {
            return false;
        }
        for input in self.nodes[a].inputs.iter() {
            if input.node == b.slot_id() {
                return true;
            }
        }
        false
    }
}

prototype!{
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

struct UnorderedSlot<T> {
    /// The id for the node (found in Graph).
    value: T,
    /// The index in indices
    index: u32,
}

impl<T> UnorderedSlot<T> {
    pub const fn new(value: T, index: u32) -> Self {
        Self { value, index }
    }
}

struct UnorderedVec<T> {
    slots: Vec<UnorderedSlot<T>>,
    unused: Vec<u32>,
    indices: Vec<u32>,
}

impl<T> UnorderedVec<T> {
    pub const fn new() -> Self {
        Self {
            slots: vec![],
            unused: vec![],
            indices: vec![],
        }
    }

    pub fn insert(&mut self, value: T) -> SlotId {
        if let Some(unused_index) = self.unused.pop() {
            self.indices[unused_index as usize] = self.slots.len() as u32;
            self.slots.push(UnorderedSlot::new(value, unused_index));
            SlotId(unused_index)
        } else {
            let index = self.indices.len() as u32;
            let slot_index = self.slots.len() as u32;
            self.slots.push(UnorderedSlot::new(value, index));
            self.indices.push(slot_index);
            SlotId(index)
        }
    }

    pub fn remove(&mut self, id: SlotId) -> T {
        let slot_index = self.indices[id.0 as usize];
        let old = self.slots.swap_remove(slot_index as usize).value;
        if slot_index as usize != self.slots.len() {
            let index = self.slots[slot_index as usize].index;
            self.indices[index as usize] = slot_index;
            self.unused.push(id.0)
        }
        old
    }

    pub fn replace(&mut self, id: SlotId, value: T) -> T {
        let slot_index = self.indices[id.0 as usize];
        self.slots[slot_index as usize].value.replace(value)
    }

    pub fn iter(&self) -> UnorderedVecIterator<T> {
        UnorderedVecIterator {
            unordered_vec: self,
            index: 0,
        }
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.slots.len()
    }
}

impl<T> std::ops::Index<SlotId> for UnorderedVec<T> {
    type Output = T;

    fn index(&self, index: SlotId) -> &Self::Output {
        let slot_index = self.indices[index.0 as usize];
        &self.slots[slot_index as usize].value
    }
}

impl<T> std::ops::IndexMut<SlotId> for UnorderedVec<T> {
    fn index_mut(&mut self, index: SlotId) -> &mut Self::Output {
        let slot_index = self.indices[index.0 as usize];
        &mut self.slots[slot_index as usize].value
    }
}

struct UnorderedVecIterator<'a, T> {
    unordered_vec: &'a UnorderedVec<T>,
    index: usize,
}

impl<'a, T> Iterator for UnorderedVecIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.unordered_vec.len() {
            return None;
        }
        let item = &self.unordered_vec.slots[self.index].value;
        self.index += 1;
        Some(item)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unordered_vec_test() {
        let mut uno = UnorderedVec::<u32>::new();
        let a = uno.insert(1);
        let b = uno.insert(2);
        let c = uno.insert(3);
        assert_eq!(uno.len(), 3);
        uno.remove(a);
        uno.remove(b);
        uno.remove(c);
        assert_eq!(uno.len(), 0);
    }

    #[test]
    fn graph_test() {
        let mut graph = Graph::<&'static str>::new();
        let d = graph.insert("d");
        let c = graph.insert_with("c", [d]);
        let b = graph.insert_with("b", [c, d]);
        let root = graph.insert_with("root", [b]);
        println!("{}", graph.check_connected(b, d, false));
    }
}
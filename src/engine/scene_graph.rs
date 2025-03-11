/*
A simple Directed Acyclic Dependency Graph for Scene ordering.
*/

use hashbrown::{HashMap, HashSet};
use super::scene::*;



#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, bytemuck::NoUninit)]
pub struct GraphId {
    id: u32,
}

impl GraphId {
    pub const NULL: Self = Self { id: 0 };
    pub const FIRST: Self = Self { id: 1 };

    pub(crate) const fn new(id: u32) -> Self {
        Self { id }
    }

    pub fn id(self) -> u32 {
        self.id
    }

    pub fn fetch_increment(&mut self) -> Self {
        let ret = *self;
        self.id = self.id + 1;
        ret
    }

    pub fn next(self) -> Self {
        Self::new(self.id + 1)
    }
}

pub struct SceneGraphNode {
    scene: SharedScene,
    active: bool,
    dependencies: HashSet<GraphId>,
    dependents: HashSet<GraphId>,
}

// Store nodes, roots, topological sort.
// When graph changes, recalculate topological sort.
pub struct SceneGraph {
    id_counter: GraphId,
    nodes: HashMap<GraphId, SceneGraphNode>,
    roots: HashSet<GraphId>,
    topological_sort: Vec<GraphId>,
    needs_sort: bool,
}

impl SceneGraphNode {
    pub fn new(scene: SharedScene, active: bool) -> Self {
        Self {
            scene,
            active,
            dependencies: HashSet::new(),
            dependents: HashSet::new(),
        }
    }
}

impl SceneGraph {
    pub fn new() -> Self {
        Self {
            // ids start at 1 so that 0 can be reserved for null.
            id_counter: GraphId::FIRST,
            nodes: HashMap::new(),
            roots: HashSet::new(),
            // Capacity of 4 elements to start with, because that seems like a reasonable number.
            topological_sort: Vec::with_capacity(4),
            needs_sort: false,
        }
    }

    /// Request that topological sort is rebuilt when it is next requested.
    fn request_sort(&mut self) {
        self.needs_sort = true;
    }

    pub fn insert_root(&mut self, scene: SharedScene, active: bool) -> GraphId {
        let id = self.id_counter.fetch_increment();
        self.nodes.insert(id, SceneGraphNode::new(scene, active));
        self.roots.insert(id);
        self.request_sort();
        id
    }

    pub fn insert_node<I>(
        &mut self,
        scene: SharedScene,
        active: bool,
        dependencies: I,
    ) -> GraphId
    where
        I: IntoIterator<Item = GraphId>,
    {
        let id = self.id_counter.fetch_increment();
        let node = self.nodes.entry(id).insert(SceneGraphNode::new(scene, active)).into_mut();
        let deps = Vec::from_iter(dependencies.into_iter());
        node.dependencies.extend(deps.iter().cloned());
        for dep in &deps {
            let dep_mut = self.nodes.get_mut(dep).expect("Failed to get dependency node. Graph was built incorrectly.");
            dep_mut.dependents.insert(id);
        }
        self.request_sort();
        id
    }
}


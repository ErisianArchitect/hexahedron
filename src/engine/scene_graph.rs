/*
A simple Directed Acyclic Dependency Graph for Scene ordering.
*/

use std::collections::VecDeque;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Dependency {
    /// The node requires the dependency to be active.
    NeedsActive(GraphId),
    /// The node only needs the correct ordering.
    NeedsOrder(GraphId),
}

impl Dependency {
    pub fn id(self) -> GraphId {
        match self {
            Dependency::NeedsActive(graph_id) => graph_id,
            Dependency::NeedsOrder(graph_id) => graph_id,
        }
    }

    /// Creates a new [Dependency] with the same variant but a new id.
    pub fn with_new_id(self, id: GraphId) -> Self {
        match self {
            Dependency::NeedsActive(_) => Dependency::NeedsActive(id),
            Dependency::NeedsOrder(_) => Dependency::NeedsOrder(id),
        }
    }

    pub fn needs_active(self) -> bool {
        matches!(self, Self::NeedsActive(_))
    }
}

pub struct SceneGraphNode {
    scene: SharedScene,
    active: bool,
    dependencies: HashSet<Dependency>,
    dependents: HashSet<Dependency>,
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
        I: IntoIterator<Item = Dependency>,
    {
        let id = self.id_counter.fetch_increment();
        let node = self.nodes.entry(id).insert(SceneGraphNode::new(scene, active)).into_mut();
        let deps = Vec::from_iter(dependencies.into_iter());
        node.dependencies.extend(deps.iter().cloned());
        for dep in deps {
            let dep_mut = self.nodes.get_mut(&dep.id()).expect("Failed to get dependency node. Graph was built incorrectly.");
            dep_mut.dependents.insert(dep.with_new_id(id));
        }
        self.request_sort();
        id
    }

    fn rebuild_topological_sort(&mut self) {
        if !self.needs_sort {
            return;
        }
        self.topological_sort.clear();
        let mut queue: VecDeque<GraphId> = VecDeque::new();
        queue.extend(&self.roots);
        let mut in_degrees = HashMap::new();

        for (&id, node) in self.nodes.iter() {
            in_degrees.insert(id, node.dependencies.len());
        }

        while let Some(id) = queue.pop_front() {
            let Some(node) = self.nodes.get(&id) else {
                panic!("Corrupt graph");
            };
            // if !node.active {
            //     // If the node isn't active, we should just skip it and do nothing with the dependents.
            //     continue;
            // }
            if node.active {
                self.topological_sort.push(id);
            }
            for dependent in node.dependents.iter() {
                if !dependent.needs_active() || node.active {
                    let Some(degrees) = in_degrees.get_mut(&dependent.id()) else {
                        panic!("Corrupt graph.");
                    };
                    *degrees -= 1;
                    if *degrees == 0 {
                        queue.push_back(dependent.id());
                    }
                }
            }
        }
        self.needs_sort = false;
    }

    pub fn topological_sort(&mut self) -> &[GraphId] {
        self.rebuild_topological_sort();
        &self.topological_sort
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestScene(&'static str);

    impl Scene for TestScene {
        
    }

    #[test]
    fn scene_graph_test() {
        let mut graph = SceneGraph::new();
        let root_0 = graph.insert_root(SharedScene::new(TestScene("Foo")), false);
        let root_1 = graph.insert_root(SharedScene::new(TestScene("Bar")), true);
        let child_0 = graph.insert_node(SharedScene::new(TestScene("Child of Foo")), true, [Dependency::NeedsActive(root_0)]);
        let child_1 = graph.insert_node(SharedScene::new(TestScene("Order of Foo")), true, [Dependency::NeedsOrder(root_0)]);
        let child_2 = graph.insert_node(SharedScene::new(TestScene("Child of Bar")), true, [Dependency::NeedsActive(root_1)]);
        let names: HashMap<GraphId, &'static str> = HashMap::from([
            (root_0, "Foo"),
            (root_1, "Bar"),
            (child_0, "Child of Foo"),
            (child_1, "Order of Foo"),
            (child_2, "Child of Bar"),
        ]);
        let top_sort = graph.topological_sort();
        for node in top_sort {
            println!("{}", names[node]);
        }
        
    }
}
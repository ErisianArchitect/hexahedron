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

pub struct GraphNode<T> {
    value: T,
    active: bool,
    dependencies: HashSet<Dependency>,
    dependents: HashSet<Dependency>,
}

// Store nodes, roots, topological sort.
// When graph changes, recalculate topological sort.
pub struct ActiveDependencyGraph<T> {
    id_counter: GraphId,
    nodes: HashMap<GraphId, GraphNode<T>>,
    roots: HashSet<GraphId>,
    topological_sort: Vec<GraphId>,
    needs_sort: bool,
}

impl<T> GraphNode<T> {
    pub fn new(value: T, active: bool) -> Self {
        Self {
            value,
            active,
            dependencies: HashSet::new(),
            dependents: HashSet::new(),
        }
    }
}

impl<T> ActiveDependencyGraph<T> {
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

    pub fn get(&self, id: GraphId) -> Option<&T> {
        self.nodes.get(&id).map(|node|&node.value)
    }

    pub fn get_mut(&mut self, id: GraphId) -> Option<&mut T> {
        self.nodes.get_mut(&id).map(|node| &mut node.value)
    }

    /// Returns whether or not the node is active.
    /// 
    /// This returns false if the node doesn't exist.
    pub fn get_active(&self, id: GraphId) -> bool {
        self.nodes.get(&id).map(|node| node.active).unwrap_or_default()
    }

    /// Sets the node's active state and returns the old state.
    pub fn set_active(&mut self, id: GraphId, active: bool) -> bool {
        let Some(node) = self.nodes.get_mut(&id) else {
            panic!("Node not found.");
        };
        std::mem::replace(&mut node.active, active)
    }

    pub fn insert_root(&mut self, value: T, active: bool) -> GraphId {
        let id = self.id_counter.fetch_increment();
        self.nodes.insert(id, GraphNode::new(value, active));
        self.roots.insert(id);
        self.request_sort();
        id
    }

    pub fn insert_node<I>(
        &mut self,
        value: T,
        active: bool,
        dependencies: I,
    ) -> GraphId
    where
        I: IntoIterator<Item = Dependency>,
    {
        let id = self.id_counter.fetch_increment();
        let node = self.nodes.entry(id).insert(GraphNode::new(value, active)).into_mut();
        let deps = Vec::from_iter(dependencies.into_iter());
        node.dependencies.extend(deps.iter().cloned());
        for dep in deps {
            let dep_mut = self.nodes.get_mut(&dep.id()).expect("Failed to get dependency node. Graph was built incorrectly.");
            dep_mut.dependents.insert(dep.with_new_id(id));
        }
        self.request_sort();
        id
    }

    pub fn remove_node(&mut self, id: GraphId) -> Option<T> {
        if let Some(node) = self.nodes.remove(&id) {
            // Remove from dependencies and dependents.
            // Dependents
            for dependent in node.dependents {
                let Some(dependent_node) = self.nodes.get_mut(&dependent.id()) else {
                    panic!("Corrupt graph.");
                };
                dependent_node.dependencies.remove(&dependent.with_new_id(id));
            }
            for dependency in node.dependencies {
                let Some(dependency_node) = self.nodes.get_mut(&dependency.id()) else {
                    panic!("Corrupt graph.");
                };
                dependency_node.dependents.remove(&dependency.with_new_id(id));
            }
            self.request_sort();
            Some(node.value)
        } else {
            None
        }
    }

    pub fn add_dependency(&mut self, id: GraphId, dependency: Dependency) {
        let Some(node) = self.nodes.get_mut(&id) else {
            panic!("Node not found.");
        };
        node.dependencies.insert(dependency);
        let Some(dependency_node) = self.nodes.get_mut(&dependency.id()) else {
            panic!("Dependency not found.");
        };
        dependency_node.dependents.insert(dependency.with_new_id(id));
        self.request_sort();
    }

    pub fn add_dependencies<I: IntoIterator<Item = Dependency>>(&mut self, id: GraphId, dependencies: I) {
        let Some(node) = self.nodes.get_mut(&id) else {
            panic!("Node not found.");
        };
        let dependencies = dependencies.into_iter().collect::<Vec<_>>();
        node.dependencies.extend(&dependencies);
        for dependency in dependencies {
            let Some(dependency_node) = self.nodes.get_mut(&dependency.id()) else {
                panic!("Dependency not found.");
            };
            dependency_node.dependents.insert(dependency.with_new_id(id));
        }
        self.request_sort();
    }

    pub fn remove_dependency(&mut self, id: GraphId, dependency: Dependency) {
        todo!()
    }

    pub fn remove_dependencies<I: IntoIterator<Item = Dependency>>(&mut self, id: GraphId, dependencies: I) {
        todo!()
    }

    pub fn remove_all_dependencies(&mut self, id: GraphId) {
        todo!()
    }

    pub fn remove_dependent(&mut self, dependent: Dependency) {
        todo!()
    }

    pub fn remove_all_dependents(&mut self) {
        todo!()
    }
    
    pub fn insert_before(&mut self, id: GraphId, target: Dependency) {
        todo!()
    }

    pub fn insert_after(&mut self, id: GraphId, target: Dependency) {
        todo!()
    }

    /// Request that topological sort is rebuilt when it is next requested.
    fn request_sort(&mut self) {
        self.needs_sort = true;
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
        let mut graph = ActiveDependencyGraph::new();
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
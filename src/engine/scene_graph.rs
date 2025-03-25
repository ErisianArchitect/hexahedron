/*
A simple Directed Acyclic Dependency Graph for Scene ordering.
*/

use std::collections::VecDeque;

use hashbrown::{HashMap, HashSet};
use super::scene::*;

#[inline(always)]
const fn map_mut_ptr<T>(refmut: &mut T) -> *mut T {
    refmut as *mut T
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, bytemuck::NoUninit)]
pub struct GraphId {
    id: u32,
}

impl GraphId {
    pub const NULL: Self = Self { id: u32::MAX };
    pub const FIRST: Self = Self { id: 0 };

    #[inline]
    pub(crate) const fn new(id: u32) -> Self {
        Self { id }
    }

    #[inline]
    pub fn id(self) -> u32 {
        self.id
    }

    #[inline]
    pub fn fetch_increment(&mut self) -> Self {
        let ret = *self;
        self.id = self.id + 1;
        ret
    }

    #[inline]
    pub fn next(self) -> Self {
        Self::new(self.id + 1)
    }

    #[inline]
    pub fn make_ids<const COUNT: usize>() -> [GraphId; COUNT] {
        std::array::from_fn(|i| GraphId::new(i as u32))
    }

    #[inline]
    pub fn make_id_range<const COUNT: usize>(start: u32, step: u32) -> [GraphId; COUNT] {
        std::array::from_fn(|i| {
            GraphId::new(((i * step as usize) + start as usize) as u32)
        })
    }
}

// I prefer being explicit, but I'll leave this macro here unless I want it later.
// #[macro_export]
// macro_rules! graph_ids {
//     ($($name:ident),+$(,)?) => {
//         let [
//             $(
//                 $name,
//             )*
//         ] = $crate::engine::scene_graph::GraphId::make_ids();
//     };
// }

// pub use crate::graph_ids;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Dependency {
    /// The node requires the dependency to be active.
    NeedsActive(GraphId),
    /// The node only needs the correct ordering.
    NeedsOrder(GraphId),
}

impl Dependency {
    #[inline]
    pub fn id(self) -> GraphId {
        match self {
            Dependency::NeedsActive(graph_id) => graph_id,
            Dependency::NeedsOrder(graph_id) => graph_id,
        }
    }

    /// Creates a new [Dependency] with the same variant but a new id.
    #[inline]
    pub fn with_new_id(self, id: GraphId) -> Self {
        match self {
            Dependency::NeedsActive(_) => Dependency::NeedsActive(id),
            Dependency::NeedsOrder(_) => Dependency::NeedsOrder(id),
        }
    }

    #[inline]
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
pub(crate) struct ActiveDependencyGraph<T> {
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
        self.request_sort();
        let Some(node) = self.nodes.get_mut(&id) else {
            panic!("Node not found.");
        };
        std::mem::replace(&mut node.active, active)
    }

    pub fn insert_root(&mut self, id: GraphId, value: T, active: bool) -> GraphId {
        self.request_sort();
        self.nodes.insert_unique_unchecked(id, GraphNode::new(value, active));
        self.roots.insert_unique_unchecked(id);
        id
    }

    pub fn insert_node<I>(
        &mut self,
        id: GraphId,
        value: T,
        active: bool,
        dependencies: I,
    ) -> GraphId
    where
        I: IntoIterator<Item = Dependency>,
    {
        self.request_sort();
        unsafe {
            let dependencies = dependencies.into_iter();
            let reserve = if self.nodes.is_empty() {
                dependencies.size_hint().0
            } else {
                (dependencies.size_hint().0 + 1) / 2
            };
            let node = map_mut_ptr(self.nodes.insert_unique_unchecked(id, GraphNode::new(value, active)).1)
                .as_mut()
                .unwrap();
            node.dependencies.reserve(reserve);
            dependencies.for_each(move |dep| {
                node.dependencies.insert(dep);
                let Some(dep_mut) = self.nodes.get_mut(&dep.id()) else {
                    panic!("Dependency not found.");
                };
                dep_mut.dependents.insert_unique_unchecked(dep.with_new_id(id));
            });
        }
        id
    }

    pub fn remove_node(&mut self, id: GraphId) -> Option<T> {
        self.request_sort();
        let Some(node) = self.nodes.remove(&id) else {
            panic!("Node not found.");
        };
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
        Some(node.value)
    }

    pub fn add_dependency(&mut self, id: GraphId, dependency: Dependency) {
        self.request_sort();
        let Some(node) = self.nodes.get_mut(&id) else {
            panic!("Node not found.");
        };
        node.dependencies.insert(dependency);
        let Some(dependency_node) = self.nodes.get_mut(&dependency.id()) else {
            panic!("Dependency not found.");
        };
        dependency_node.dependents.insert(dependency.with_new_id(id));
    }

    pub fn add_dependencies<I: IntoIterator<Item = Dependency>>(&mut self, id: GraphId, dependencies: I) {
        self.request_sort();
        unsafe {
            let dependencies = dependencies.into_iter();
            let reserve = if self.nodes.is_empty() {
                dependencies.size_hint().0
            } else {
                (dependencies.size_hint().0 + 1) / 2
            };
            let node_mut = self
                .nodes
                .get_mut(&id)
                .map(map_mut_ptr)
                .expect("Node not found.")
                .as_mut()
                .unwrap();
            node_mut.dependencies.reserve(reserve);
            dependencies.for_each(move |dep| {
                node_mut.dependencies.insert(dep);
                let Some(dep_mut) = self.nodes.get_mut(&dep.id()) else {
                    panic!("Dependency not found.");
                };
                dep_mut.dependents.insert(dep.with_new_id(id));
            });
        }
    }

    pub fn remove_dependency(&mut self, id: GraphId, dependency: Dependency) {
        self.request_sort();
        if let Some(node_mut) = self.nodes.get_mut(&id) {
            node_mut.dependencies.remove(&dependency);
            if node_mut.dependencies.is_empty() {
                self.roots.insert(id);
            }
        }
        if let Some(dep_mut) = self.nodes.get_mut(&dependency.id()) {
            dep_mut.dependents.remove(&dependency.with_new_id(id));
        }
    }

    pub fn remove_dependencies<I: IntoIterator<Item = Dependency>>(&mut self, id: GraphId, dependencies: I) {
        self.request_sort();
        unsafe {
            let node_mut = self
                .nodes
                .get_mut(&id)
                .map(map_mut_ptr)
                .expect("Node not found.")
                .as_mut()
                .unwrap();
            dependencies.into_iter().for_each(move |dep| {
                debug_assert_ne!(id, dep.id(), "Corrupt graph.");
                node_mut.dependencies.remove(&dep);
                let Some(dep_mut) = self.nodes.get_mut(&dep.id()) else {
                    panic!("Dependency not found.");
                };
                dep_mut.dependents.remove(&dep.with_new_id(id));
            });
        }
    }

    pub fn remove_all_dependencies(&mut self, id: GraphId) {
        self.request_sort();
        unsafe {
            let node_mut = self
                .nodes
                .get_mut(&id)
                .map(map_mut_ptr)
                .expect("Node not found.")
                .as_mut()
                .unwrap();
            node_mut.dependencies.drain().for_each(move |dep| {
                debug_assert_ne!(id, dep.id(), "Corrupt graph.");
                let Some(dep_mut) = self.nodes.get_mut(&dep.id()) else {
                    panic!("Dependency not found.");
                };
                dep_mut.dependents.remove(&dep.with_new_id(id));
            });
        }
    }

    pub fn remove_dependent(&mut self, id: GraphId, dependent: Dependency) {
        self.request_sort();
        let Some(node_mut) = self.nodes.get_mut(&id) else {
            panic!("Node not found.");
        };
        node_mut.dependents.remove(&dependent);
        let Some(dep_mut) = self.nodes.get_mut(&dependent.id()) else {
            panic!("Dependent not found.");
        };
        dep_mut.dependencies.remove(&dependent.with_new_id(id));
    }

    pub fn remove_dependents<I: IntoIterator<Item = Dependency>>(&mut self, id: GraphId, dependents: I) {
        self.request_sort();
        unsafe {
            let node_mut = self
                .nodes
                .get_mut(&id)
                .map(map_mut_ptr)
                .expect("Node not found.")
                .as_mut()
                .unwrap();
            dependents.into_iter().for_each(move |dep| {
                node_mut.dependents.remove(&dep);
                let Some(dep_mut) = self.nodes.get_mut(&dep.id()) else {
                    panic!("Dependency not found.");
                };
                dep_mut.dependencies.remove(&dep.with_new_id(id));
            });
        }
    }

    pub fn remove_all_dependents(&mut self, id: GraphId) {
        self.request_sort();
        unsafe {
            let node_mut = self
                .nodes
                .get_mut(&id)
                .map(map_mut_ptr)
                .expect("Node not found.")
                .as_mut()
                .unwrap();
            node_mut.dependents.drain().for_each(move |dep| {
                let Some(dep_mut) = self.nodes.get_mut(&dep.id()) else {
                    panic!("Dependency not found.");
                };
                dep_mut.dependencies.remove(&dep.with_new_id(id));
            });
        }
    }
    
    pub fn insert_before(&mut self, before: GraphId, target: GraphId, needs_active: bool) {
        let dep = if needs_active {
            Dependency::NeedsActive(before)
        } else {
            Dependency::NeedsOrder(before)
        };
        self.add_dependency(target, dep);
    }

    pub fn insert_after(&mut self, after: GraphId, target: GraphId, needs_active: bool) {
        let dep = if needs_active {
            Dependency::NeedsActive(target)
        } else {
            Dependency::NeedsOrder(target)
        };
        self.add_dependency(after, dep);
    }

    /// Request that topological sort is rebuilt when it is next requested.
    #[inline(always)]
    fn request_sort(&mut self) {
        self.needs_sort = true;
    }

    #[inline(always)]
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
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }

        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
    }

    #[test]
    fn scene_graph_test() {
        let ids = GraphId::make_ids();
        /* Potential Scenes
        - Main Menu
        - Loading Screen
        - Pause Menu
        - Game Screen
        */
        let [
            root_0,
            root_1,
            child_0,
            child_1,
            child_2
        ] = ids;
        println!("IDS: {:#?}", ids);
        println!("********************************");
        println!("IDs: {:#?}", GraphId::make_id_range::<3>(0, 3));
        println!("IDs: {:#?}", GraphId::make_id_range::<3>(1, 3));
        println!("IDs: {:#?}", GraphId::make_id_range::<3>(2, 3));
        return;
        let mut graph = ActiveDependencyGraph::new();
        graph.insert_root(root_0, SharedScene::new(TestScene("Foo")), false);
        graph.insert_root(root_1, SharedScene::new(TestScene("Bar")), true);
        graph.insert_node(child_0, SharedScene::new(TestScene("Child of Foo")), true, [Dependency::NeedsActive(root_0)]);
        graph.insert_node(child_1, SharedScene::new(TestScene("Order of Foo")), true, [Dependency::NeedsOrder(root_0)]);
        graph.insert_node(child_2, SharedScene::new(TestScene("Child of Bar")), true, [Dependency::NeedsActive(root_1)]);
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

macro_rules! prototype { ($($_:tt)*) => {} }

prototype!(
    let mut scene_graph = SceneGraph::new();
    // Scenes by default iterate in the order they were inserted.
    scene_graph.insert(key1, /* scene creation */, active_state1);
    scene_graph.insert(key1, /* scene creation */, active_state2);
    for scene_mut in scene_graph.update_iter() {
        // ...
    }
    for scene_mut in scene_graph.render_iter() {
        // ...
    }
    for scene_mut in scene_graph.process_event_iter() {
        // ...
    }
    for scene_mut in scene_graph.process_window_event_iter() {
        // ...
    }
    for scene_mut in scene_graph.gamepad_event_iter() {

    }
    graph!(
        root->(
            (
                (
                    (child0, child1)->grandchild0,
                    (child1, child2)->grandchild1,
                    (child0, child1, child2)->(grandchild2, grandchild3),
                )->great_grandchild0,
                (child0, child2)->grandchild4->great_grandchild1,
            )->great_great_grandchild0,
        )
    );
);
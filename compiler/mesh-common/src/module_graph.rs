//! Module graph types for the Mesh compiler.
//!
//! Provides the core data structures used by all module-system phases:
//! [`ModuleId`], [`ModuleInfo`], [`ModuleGraph`], and [`CycleError`].

use std::collections::VecDeque;
use std::fmt;
use std::path::PathBuf;

use rustc_hash::FxHashMap;

/// A unique identifier for a module within a compilation unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModuleId(pub u32);

/// Metadata about a single module in the module graph.
#[derive(Debug)]
pub struct ModuleInfo {
    /// Unique identifier for this module.
    pub id: ModuleId,
    /// PascalCase module name, e.g. `"Math.Vector"`.
    pub name: String,
    /// Path relative to the project root, e.g. `"math/vector.mpl"`.
    pub path: PathBuf,
    /// Modules that this module depends on (via `import` statements).
    pub dependencies: Vec<ModuleId>,
    /// Whether this module is the project entry point (`main.mpl`).
    pub is_entry: bool,
}

/// Error returned when a dependency cycle is detected in the module graph.
#[derive(Debug, Clone)]
pub struct CycleError {
    /// The module names forming the cycle, e.g. `["A", "B", "C", "A"]`.
    pub cycle_path: Vec<String>,
}

impl fmt::Display for CycleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cycle_path.join(" -> "))
    }
}

/// A directed graph of modules and their dependencies.
///
/// Modules are stored in insertion order and identified by [`ModuleId`].
/// Name-based lookup is provided via an internal hash map.
#[derive(Debug)]
pub struct ModuleGraph {
    /// All modules in the graph, indexed by `ModuleId.0`.
    pub modules: Vec<ModuleInfo>,
    /// Maps PascalCase module names to their [`ModuleId`].
    name_to_id: FxHashMap<String, ModuleId>,
}

impl ModuleGraph {
    /// Create an empty module graph.
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
            name_to_id: FxHashMap::default(),
        }
    }

    /// Add a module to the graph and return its assigned [`ModuleId`].
    ///
    /// The ID is assigned sequentially starting from 0.
    pub fn add_module(&mut self, name: String, path: PathBuf, is_entry: bool) -> ModuleId {
        let id = ModuleId(self.modules.len() as u32);
        self.name_to_id.insert(name.clone(), id);
        self.modules.push(ModuleInfo {
            id,
            name,
            path,
            dependencies: Vec::new(),
            is_entry,
        });
        id
    }

    /// Look up a module by its PascalCase name.
    pub fn resolve(&self, name: &str) -> Option<ModuleId> {
        self.name_to_id.get(name).copied()
    }

    /// Record that module `from` depends on module `to`.
    /// Duplicate and self-dependencies are ignored.
    pub fn add_dependency(&mut self, from: ModuleId, to: ModuleId) {
        if from == to {
            return;
        }
        let deps = &mut self.modules[from.0 as usize].dependencies;
        if !deps.contains(&to) {
            deps.push(to);
        }
    }

    /// Return the number of modules in the graph.
    pub fn module_count(&self) -> usize {
        self.modules.len()
    }

    /// Get a reference to a module by its [`ModuleId`].
    pub fn get(&self, id: ModuleId) -> &ModuleInfo {
        &self.modules[id.0 as usize]
    }
}

/// Topological sort of the module graph using Kahn's algorithm.
///
/// Returns modules in dependency order: leaf modules (no dependencies) first,
/// entry module last. Uses alphabetical tie-breaking for determinism.
///
/// Returns `Err(CycleError)` if the graph contains a dependency cycle.
pub fn topological_sort(graph: &ModuleGraph) -> Result<Vec<ModuleId>, CycleError> {
    let n = graph.modules.len();
    // in_degree[i] = number of module i's dependencies not yet processed.
    // Modules with 0 dependencies compile first; the entry module (many deps) goes last.
    let mut in_degree: Vec<u32> = graph
        .modules
        .iter()
        .map(|m| m.dependencies.len() as u32)
        .collect();

    // Seed queue with modules that have no dependencies.
    let mut ready: Vec<ModuleId> = (0..n)
        .filter(|&i| in_degree[i] == 0)
        .map(|i| ModuleId(i as u32))
        .collect();
    // Sort alphabetically by module name for determinism.
    ready.sort_by(|a, b| {
        graph.modules[a.0 as usize]
            .name
            .cmp(&graph.modules[b.0 as usize].name)
    });

    let mut queue = VecDeque::from(ready);
    let mut order = Vec::with_capacity(n);

    while let Some(id) = queue.pop_front() {
        order.push(id);
        // For every other module that depends on `id`, decrement its in_degree.
        let mut newly_ready = Vec::new();
        for (i, module) in graph.modules.iter().enumerate() {
            if in_degree[i] > 0 && module.dependencies.contains(&id) {
                in_degree[i] -= 1;
                if in_degree[i] == 0 {
                    newly_ready.push(ModuleId(i as u32));
                }
            }
        }
        // Sort newly ready modules alphabetically for determinism.
        newly_ready.sort_by(|a, b| {
            graph.modules[a.0 as usize]
                .name
                .cmp(&graph.modules[b.0 as usize].name)
        });
        queue.extend(newly_ready);
    }

    if order.len() == n {
        Ok(order)
    } else {
        Err(CycleError {
            cycle_path: extract_cycle_path(graph, &in_degree),
        })
    }
}

/// Extract a cycle path from modules that remain unprocessed (in_degree > 0).
///
/// Follows dependency edges among unprocessed modules until a module is revisited,
/// forming the cycle path. Returns module names ending with the repeated name.
fn extract_cycle_path(graph: &ModuleGraph, in_degree: &[u32]) -> Vec<String> {
    // Find any module still in a cycle (in_degree > 0).
    let start = match (0..graph.modules.len()).find(|&i| in_degree[i] > 0) {
        Some(i) => i,
        None => return Vec::new(),
    };

    let mut path = Vec::new();
    let mut visited = vec![false; graph.modules.len()];
    let mut current = start;

    loop {
        if visited[current] {
            // Found the cycle start -- trim path to just the cycle portion.
            let cycle_start_name = &graph.modules[current].name;
            let cycle_begin = path
                .iter()
                .position(|name: &String| name == cycle_start_name)
                .unwrap_or(0);
            let mut cycle: Vec<String> = path[cycle_begin..].to_vec();
            cycle.push(cycle_start_name.clone());
            return cycle;
        }

        visited[current] = true;
        path.push(graph.modules[current].name.clone());

        // Follow a dependency edge to another unprocessed module.
        let next = graph.modules[current]
            .dependencies
            .iter()
            .find(|dep| in_degree[dep.0 as usize] > 0);

        match next {
            Some(dep) => current = dep.0 as usize,
            None => {
                // Should not happen if in_degree > 0, but be safe.
                path.push(graph.modules[current].name.clone());
                return path;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_resolve() {
        let mut graph = ModuleGraph::new();
        let id_a = graph.add_module("Math.Vector".into(), "math/vector.mpl".into(), false);
        let id_b = graph.add_module("Utils".into(), "utils.mpl".into(), false);

        assert_eq!(graph.resolve("Math.Vector"), Some(id_a));
        assert_eq!(graph.resolve("Utils"), Some(id_b));
        assert_ne!(id_a, id_b);
        assert_eq!(graph.module_count(), 2);
    }

    #[test]
    fn test_resolve_unknown() {
        let graph = ModuleGraph::new();
        assert_eq!(graph.resolve("Nonexistent"), None);
    }

    #[test]
    fn test_add_dependency() {
        let mut graph = ModuleGraph::new();
        let id_a = graph.add_module("A".into(), "a.mpl".into(), false);
        let id_b = graph.add_module("B".into(), "b.mpl".into(), false);

        graph.add_dependency(id_a, id_b);

        let module_a = graph.get(id_a);
        assert_eq!(module_a.dependencies, vec![id_b]);

        // B should have no dependencies
        let module_b = graph.get(id_b);
        assert!(module_b.dependencies.is_empty());
    }

    #[test]
    fn test_entry_module() {
        let mut graph = ModuleGraph::new();
        let entry_id = graph.add_module("Main".into(), "main.mpl".into(), true);
        let lib_id = graph.add_module("Lib".into(), "lib.mpl".into(), false);

        assert!(graph.get(entry_id).is_entry);
        assert!(!graph.get(lib_id).is_entry);
    }

    // ── Topological sort tests ──────────────────────────────────────────

    #[test]
    fn test_toposort_linear() {
        // A depends on B, B depends on C. Result: [C, B, A].
        let mut graph = ModuleGraph::new();
        let id_a = graph.add_module("A".into(), "a.mpl".into(), false);
        let id_b = graph.add_module("B".into(), "b.mpl".into(), false);
        let id_c = graph.add_module("C".into(), "c.mpl".into(), false);

        graph.add_dependency(id_a, id_b);
        graph.add_dependency(id_b, id_c);

        let order = topological_sort(&graph).unwrap();
        let names: Vec<&str> = order
            .iter()
            .map(|id| graph.get(*id).name.as_str())
            .collect();
        assert_eq!(names, vec!["C", "B", "A"]);
    }

    #[test]
    fn test_toposort_independent() {
        // A, B, C with no deps. Result: [A, B, C] (alphabetical).
        let mut graph = ModuleGraph::new();
        graph.add_module("C".into(), "c.mpl".into(), false);
        graph.add_module("A".into(), "a.mpl".into(), false);
        graph.add_module("B".into(), "b.mpl".into(), false);

        let order = topological_sort(&graph).unwrap();
        let names: Vec<&str> = order
            .iter()
            .map(|id| graph.get(*id).name.as_str())
            .collect();
        assert_eq!(names, vec!["A", "B", "C"]);
    }

    #[test]
    fn test_toposort_diamond() {
        // A deps [B, C], B deps [D], C deps [D]. Result: [D, B, C, A].
        let mut graph = ModuleGraph::new();
        let id_a = graph.add_module("A".into(), "a.mpl".into(), false);
        let id_b = graph.add_module("B".into(), "b.mpl".into(), false);
        let id_c = graph.add_module("C".into(), "c.mpl".into(), false);
        let id_d = graph.add_module("D".into(), "d.mpl".into(), false);

        graph.add_dependency(id_a, id_b);
        graph.add_dependency(id_a, id_c);
        graph.add_dependency(id_b, id_d);
        graph.add_dependency(id_c, id_d);

        let order = topological_sort(&graph).unwrap();
        let names: Vec<&str> = order
            .iter()
            .map(|id| graph.get(*id).name.as_str())
            .collect();
        assert_eq!(names, vec!["D", "B", "C", "A"]);
    }

    #[test]
    fn test_toposort_cycle() {
        // A deps [B], B deps [C], C deps [A]. Returns Err with cycle path.
        let mut graph = ModuleGraph::new();
        let id_a = graph.add_module("A".into(), "a.mpl".into(), false);
        let id_b = graph.add_module("B".into(), "b.mpl".into(), false);
        let id_c = graph.add_module("C".into(), "c.mpl".into(), false);

        graph.add_dependency(id_a, id_b);
        graph.add_dependency(id_b, id_c);
        graph.add_dependency(id_c, id_a);

        let err = topological_sort(&graph).unwrap_err();
        assert!(
            err.cycle_path.len() >= 3,
            "cycle path should have at least 3 entries"
        );
        // The cycle path should contain all three module names and repeat one.
        assert!(err.cycle_path.contains(&"A".to_string()));
        assert!(err.cycle_path.contains(&"B".to_string()));
        assert!(err.cycle_path.contains(&"C".to_string()));
        // Last element should equal one of the earlier elements (cycle).
        assert_eq!(
            err.cycle_path.first(),
            Some(&err.cycle_path.last().unwrap().clone())
                .as_ref()
                .map(|s| *s)
        );
    }

    #[test]
    fn test_toposort_self_dep_ignored() {
        // Self-dependencies are silently dropped, so A has no deps and sorts fine.
        let mut graph = ModuleGraph::new();
        let id_a = graph.add_module("A".into(), "a.mpl".into(), false);

        graph.add_dependency(id_a, id_a);

        let order = topological_sort(&graph).unwrap();
        assert_eq!(order.len(), 1);
        assert_eq!(order[0], id_a);
    }

    #[test]
    fn test_toposort_entry_last() {
        // Entry module (Main) depends on Utils and Math. Result: [Math, Utils, Main].
        let mut graph = ModuleGraph::new();
        let id_main = graph.add_module("Main".into(), "main.mpl".into(), true);
        let id_utils = graph.add_module("Utils".into(), "utils.mpl".into(), false);
        let id_math = graph.add_module("Math".into(), "math.mpl".into(), false);

        graph.add_dependency(id_main, id_utils);
        graph.add_dependency(id_main, id_math);

        let order = topological_sort(&graph).unwrap();
        let names: Vec<&str> = order
            .iter()
            .map(|id| graph.get(*id).name.as_str())
            .collect();
        assert_eq!(names, vec!["Math", "Utils", "Main"]);
    }
}

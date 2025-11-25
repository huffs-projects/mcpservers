use crate::plugins::registry::PluginRegistry;
use std::collections::{HashMap, HashSet, VecDeque};

/// Directed graph representing plugin load order and dependencies
pub struct PluginGraph {
    nodes: HashSet<String>,
    edges: HashMap<String, Vec<String>>, // from -> to
    reverse_edges: HashMap<String, Vec<String>>, // to -> from
}

impl PluginGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashSet::new(),
            edges: HashMap::new(),
            reverse_edges: HashMap::new(),
        }
    }

    /// Build graph from plugin registry
    pub fn from_registry(registry: &PluginRegistry) -> Self {
        let mut graph = Self::new();
        
        for plugin in registry.get_all_plugins() {
            graph.add_node(plugin.name.clone());
            
            for dep in &plugin.dependencies {
                graph.add_node(dep.name.clone());
                graph.add_edge(dep.name.clone(), plugin.name.clone());
            }
        }
        
        graph
    }

    /// Add a node to the graph
    pub fn add_node(&mut self, node: String) {
        self.nodes.insert(node.clone());
        self.edges.entry(node.clone()).or_insert_with(Vec::new);
        self.reverse_edges.entry(node.clone()).or_insert_with(Vec::new);
    }

    /// Add an edge (dependency) from -> to
    pub fn add_edge(&mut self, from: String, to: String) {
        self.edges.entry(from.clone()).or_insert_with(Vec::new).push(to.clone());
        self.reverse_edges.entry(to.clone()).or_insert_with(Vec::new).push(from.clone());
    }

    /// Detect cycles in the dependency graph
    pub fn detect_cycles(&self) -> Vec<Vec<String>> {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let mut path = Vec::new();

        for node in &self.nodes {
            if !visited.contains(node) {
                self.dfs_cycle(node, &mut visited, &mut rec_stack, &mut path, &mut cycles);
            }
        }

        cycles
    }

    fn dfs_cycle(
        &self,
        node: &String,
        visited: &mut HashSet<String>,
        rec_stack: &mut HashSet<String>,
        path: &mut Vec<String>,
        cycles: &mut Vec<Vec<String>>,
    ) {
        visited.insert(node.clone());
        rec_stack.insert(node.clone());
        path.push(node.clone());

        if let Some(neighbors) = self.edges.get(node) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) {
                    self.dfs_cycle(neighbor, visited, rec_stack, path, cycles);
                } else if rec_stack.contains(neighbor) {
                    // Found a cycle
                    if let Some(start_idx) = path.iter().position(|n| n == neighbor) {
                        cycles.push(path[start_idx..].to_vec());
                    }
                }
            }
        }

        rec_stack.remove(node);
        path.pop();
    }

    /// Topological sort to determine load order
    pub fn topological_sort(&self) -> Result<Vec<String>, Vec<String>> {
        let mut in_degree = HashMap::new();
        
        // Initialize in-degrees
        for node in &self.nodes {
            in_degree.insert(node.clone(), 0);
        }
        
        // Calculate in-degrees
        for edges in self.edges.values() {
            for to in edges {
                *in_degree.entry(to.clone()).or_insert(0) += 1;
            }
        }

        // Kahn's algorithm
        let mut queue = VecDeque::new();
        for (node, degree) in &in_degree {
            if *degree == 0 {
                queue.push_back(node.clone());
            }
        }

        let mut result = Vec::new();
        let mut processed = 0;

        while let Some(node) = queue.pop_front() {
            result.push(node.clone());
            processed += 1;

            if let Some(neighbors) = self.edges.get(&node) {
                for neighbor in neighbors {
                    let degree = in_degree.get_mut(neighbor).unwrap();
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }

        if processed != self.nodes.len() {
            // Cycle detected
            Err(self.detect_cycles().into_iter().flatten().collect())
        } else {
            Ok(result)
        }
    }

    /// Get plugins that should load before a given plugin
    pub fn get_prerequisites(&self, plugin: &str) -> Vec<String> {
        self.reverse_edges
            .get(plugin)
            .cloned()
            .unwrap_or_default()
    }

    /// Get plugins that depend on a given plugin
    pub fn get_dependents(&self, plugin: &str) -> Vec<String> {
        self.edges
            .get(plugin)
            .cloned()
            .unwrap_or_default()
    }
}

impl Default for PluginGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_node() {
        let mut graph = PluginGraph::new();
        graph.add_node("plugin1".to_string());
        assert!(graph.nodes.contains("plugin1"));
    }

    #[test]
    fn test_add_edge() {
        let mut graph = PluginGraph::new();
        graph.add_node("plugin1".to_string());
        graph.add_node("plugin2".to_string());
        graph.add_edge("plugin1".to_string(), "plugin2".to_string());
        
        let deps = graph.get_dependents("plugin1");
        assert_eq!(deps.len(), 1);
        assert_eq!(deps[0], "plugin2");
    }

    #[test]
    fn test_detect_cycles() {
        let mut graph = PluginGraph::new();
        graph.add_node("plugin1".to_string());
        graph.add_node("plugin2".to_string());
        graph.add_edge("plugin1".to_string(), "plugin2".to_string());
        graph.add_edge("plugin2".to_string(), "plugin1".to_string());
        
        let cycles = graph.detect_cycles();
        assert!(!cycles.is_empty(), "Should detect cycle");
    }

    #[test]
    fn test_topological_sort() {
        let mut graph = PluginGraph::new();
        graph.add_node("plugin1".to_string());
        graph.add_node("plugin2".to_string());
        graph.add_node("plugin3".to_string());
        graph.add_edge("plugin1".to_string(), "plugin2".to_string());
        graph.add_edge("plugin2".to_string(), "plugin3".to_string());
        
        let sorted = graph.topological_sort();
        assert!(sorted.is_ok(), "Should sort acyclic graph");
        let sorted = sorted.unwrap();
        assert!(sorted.contains(&"plugin1".to_string()));
        assert!(sorted.contains(&"plugin2".to_string()));
        assert!(sorted.contains(&"plugin3".to_string()));
    }
}


/*
	graph
	This problem requires you to implement a basic graph functio
*/
// I AM NOT DONE

use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeNotInGraph;

impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

/// 无向加权图结构
/// 邻接表：key=节点名，value=邻接节点列表（(节点名, 边权重)）
pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

/// 图通用接口
pub trait Graph {
    /// 创建新图
    fn new() -> Self;

    /// 获取邻接表可变引用
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;

    /// 获取邻接表不可变引用
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;

    /// 添加节点（已存在则返回false，新增返回true）
    fn add_node(&mut self, node: &str) -> bool {
        // 节点已存在，返回false
        if self.contains(node) {
            return false;
        }
        // 新增节点，初始化空邻接列表
        self.adjacency_table_mutable()
            .insert(node.to_string(), Vec::new());
        true
    }

    /// 添加无向边（src <-> dest，权重weight）
    fn add_edge(&mut self, edge: (&str, &str, i32));

    /// 判断节点是否存在
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }

    /// 获取所有节点集合
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }

    /// 获取所有边列表（(起点, 终点, 权重)）
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}

impl Graph for UndirectedGraph {
    /// 创建空的无向图
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }

    /// 获取邻接表可变引用
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    /// 获取邻接表不可变引用
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }

    /// 添加无向边：需双向存储（src→dest 和 dest→src）
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (src, dest, weight) = edge;

        // 步骤1：确保两个节点都存在（不存在则自动添加）
        self.add_node(src);
        self.add_node(dest);

        // 步骤2：向src的邻接列表添加dest（权重weight）
        self.adjacency_table_mutable()
            .get_mut(&src.to_string())
            .unwrap() // 节点已添加，unwrap安全
            .push((dest.to_string(), weight));

        // 步骤3：向dest的邻接列表添加src（权重weight）（无向图核心）
        self.adjacency_table_mutable()
            .get_mut(&dest.to_string())
            .unwrap()
            .push((src.to_string(), weight));
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;

    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];

        for edge in expected_edges.iter() {
            assert!(graph.edges().contains(edge), "Missing edge: {:?}", edge);
        }
    }

    #[test]
    fn test_add_node() {
        let mut graph = UndirectedGraph::new();
        // 新增节点返回true
        assert_eq!(graph.add_node("a"), true);
        // 重复添加返回false
        assert_eq!(graph.add_node("a"), false);
        // 验证节点存在
        assert!(graph.contains("a"));
        // 验证节点列表包含"a"
        assert!(graph.nodes().contains(&&String::from("a")));
    }

    #[test]
    fn test_empty_graph() {
        let graph = UndirectedGraph::new();
        // 空图无节点、无边
        assert!(graph.nodes().is_empty());
        assert!(graph.edges().is_empty());
    }
}

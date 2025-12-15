/*
	dfs
	This problem requires you to implement a basic DFS traversal
*/

// I AM NOT DONE
use std::collections::HashSet;

struct Graph {
    adj: Vec<Vec<usize>>, // 邻接表：adj[src] 存储与src相连的所有节点
}

impl Graph {
    // 创建包含n个顶点的空图
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // 向无向图添加边（双向）
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src);
    }

    // DFS递归辅助函数：核心遍历逻辑
    fn dfs_util(&self, v: usize, visited: &mut HashSet<usize>, visit_order: &mut Vec<usize>) {
        // 1. 标记当前节点为已访问，并记录访问顺序
        visited.insert(v);
        visit_order.push(v);

        // 2. 遍历当前节点的所有邻接节点
        for &neighbor in &self.adj[v] {
            // 3. 若邻接节点未访问，则递归访问
            if !visited.contains(&neighbor) {
                self.dfs_util(neighbor, visited, visit_order);
            }
        }
    }

    // 执行DFS，返回节点访问顺序（对外接口）
    fn dfs(&self, start: usize) -> Vec<usize> {
        // 边界检查：起始节点超出图的顶点范围时返回空数组
        if start >= self.adj.len() {
            return vec![];
        }

        let mut visited = HashSet::new(); // 记录已访问节点（避免环/重复访问）
        let mut visit_order = Vec::new(); // 存储访问顺序
        self.dfs_util(start, &mut visited, &mut visit_order);
        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dfs_simple() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_dfs_with_cycle() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 3); // 自环（不影响DFS，会被visited过滤）

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_dfs_disconnected_graph() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(3, 4); // 不连通的子图

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]); // 仅访问连通分量
        let visit_order_disconnected = graph.dfs(3);
        assert_eq!(visit_order_disconnected, vec![3, 4]); // 访问另一个连通分量
    }

    // 额外测试：单节点图
    #[test]
    fn test_dfs_single_node() {
        let graph = Graph::new(1);
        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0]);
    }

    // 额外测试：起始节点越界
    #[test]
    fn test_dfs_invalid_start() {
        let graph = Graph::new(3);
        let visit_order = graph.dfs(5);
        assert_eq!(visit_order, vec![]);
    }
}

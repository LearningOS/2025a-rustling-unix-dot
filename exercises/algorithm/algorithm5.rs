/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/

//I AM NOT DONE
use std::collections::VecDeque;

// 无向图结构定义
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

    // 向图中添加无向边（src <-> dest）
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src);
    }

    // 执行广度优先搜索，返回节点访问顺序
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        // 边界检查：起始节点超出图的顶点范围
        if start >= self.adj.len() {
            return vec![];
        }

        // 初始化访问标记：false表示未访问，true表示已访问
        let mut visited = vec![false; self.adj.len()];
        // BFS核心队列：存储待访问的节点
        let mut queue = VecDeque::new();
        // 存储访问顺序的结果数组
        let mut visit_order = vec![];

        // 起始节点入队并标记为已访问
        visited[start] = true;
        queue.push_back(start);

        // 队列非空时循环处理
        while let Some(current) = queue.pop_front() {
            // 记录当前节点的访问顺序
            visit_order.push(current);

            // 遍历当前节点的所有邻接节点
            for &neighbor in &self.adj[current] {
                // 未访问过的邻接节点入队并标记
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back(neighbor);
                }
            }
        }

        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}

/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

//I AM NOT DONE
use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    // 递归插入节点（内部辅助方法）
    fn insert(&mut self, value: T) {
        match value.cmp(&self.value) {
            // 小于当前节点值：插入左子树
            Ordering::Less => match &mut self.left {
                Some(left_node) => left_node.insert(value),
                None => self.left = Some(Box::new(TreeNode::new(value))),
            },
            // 大于当前节点值：插入右子树
            Ordering::Greater => match &mut self.right {
                Some(right_node) => right_node.insert(value),
                None => self.right = Some(Box::new(TreeNode::new(value))),
            },
            // 等于：不插入（BST不存储重复值）
            Ordering::Equal => (),
        }
    }

    // 递归搜索节点（内部辅助方法）
    fn search(&self, value: &T) -> bool {
        match value.cmp(&self.value) {
            // 找到匹配值
            Ordering::Equal => true,
            // 小于当前值：搜索左子树
            Ordering::Less => self.left.as_ref().map_or(false, |left| left.search(value)),
            // 大于当前值：搜索右子树
            Ordering::Greater => self.right.as_ref().map_or(false, |right| right.search(value)),
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // 插入值到BST（对外接口）
    fn insert(&mut self, value: T) {
        match &mut self.root {
            // 根节点为空：直接创建根节点
            None => self.root = Some(Box::new(TreeNode::new(value))),
            // 根节点存在：调用节点的插入方法
            Some(root_node) => root_node.insert(value),
        }
    }

    // 搜索值是否存在（对外接口）
    fn search(&self, value: T) -> bool {
        self.root.as_ref().map_or(false, |root| root.search(&value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        // 初始空树，搜索不存在的值
        assert_eq!(bst.search(1), false);

        // 插入节点
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        // 验证存在的节点
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        // 验证不存在的节点
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        // 插入重复值
        bst.insert(1);
        bst.insert(1);

        // 验证值存在
        assert_eq!(bst.search(1), true);

        // 验证没有创建重复节点
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}

/*
	double linked list reverse
	This problem requires you to reverse a doubly linked list
*/
// I AM NOT DONE

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            prev: None,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        node.prev = self.end;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

    /// 反转双向链表
    pub fn reverse(&mut self) {
        // 空链表或只有一个节点时无需反转
        if self.length <= 1 {
            return;
        }

        let mut current = self.start;
        // 交换链表的头尾指针
        std::mem::swap(&mut self.start, &mut self.end);

        // 遍历所有节点，交换prev和next指针
        while let Some(node_ptr) = current {
            unsafe {
                let node = node_ptr.as_ptr();
                // 交换当前节点的prev和next
                std::mem::swap(&mut (*node).prev, &mut (*node).next);
                // 移动到原next（现在的prev）节点继续处理
                current = (*node).prev;
            }
        }
    }
}

// 实现Drop trait防止内存泄漏
impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.start;
        while let Some(node_ptr) = current {
            unsafe {
                // 保存下一个节点的指针
                current = (*node_ptr.as_ptr()).next;
                // 将裸指针转回Box并释放内存
                let _ = Box::from_raw(node_ptr.as_ptr());
            }
        }
    }
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut current = self.start;
        let mut first = true;
        while let Some(node_ptr) = current {
            if !first {
                write!(f, ", ")?;
            }
            // 安全访问节点值
            write!(f, "{}", unsafe { &(*node_ptr.as_ptr()).val })?;
            // 移动到下一个节点
            current = unsafe { (*node_ptr.as_ptr()).next };
            first = false;
        }
        Ok(())
    }
}

// 移除Node的Display实现（避免递归溢出，改用LinkedList的迭代实现）
// impl<T> Display for Node<T> where T: Display { ... }

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_reverse_linked_list_1() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![2, 3, 5, 11, 9, 7];
        let reverse_vec = vec![7, 9, 11, 5, 3, 2];
        
        for &val in &original_vec {
            list.add(val);
        }
        
        println!("Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        
        for i in 0..original_vec.len() {
            assert_eq!(reverse_vec[i], *list.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_reverse_linked_list_2() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![34, 56, 78, 25, 90, 10, 19, 34, 21, 45];
        let reverse_vec = vec![45, 21, 34, 19, 10, 90, 25, 78, 56, 34];
        
        for &val in &original_vec {
            list.add(val);
        }
        
        println!("Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        
        for i in 0..original_vec.len() {
            assert_eq!(reverse_vec[i], *list.get(i as i32).unwrap());
        }
    }

    // 测试空链表和单节点链表反转
    #[test]
    fn test_reverse_edge_cases() {
        // 空链表
        let mut empty_list = LinkedList::<i32>::new();
        empty_list.reverse();
        assert_eq!(empty_list.length, 0);
        assert!(empty_list.get(0).is_none());

        // 单节点链表
        let mut single_node_list = LinkedList::<i32>::new();
        single_node_list.add(42);
        single_node_list.reverse();
        assert_eq!(*single_node_list.get(0).unwrap(), 42);
        assert_eq!(single_node_list.length, 1);
    }
}

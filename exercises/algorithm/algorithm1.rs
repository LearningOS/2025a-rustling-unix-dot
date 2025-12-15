/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/
// I AM NOT DONE

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
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

    /// 合并两个有序单链表为一个新的有序单链表
    pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self
    where
        T: Ord,
    {
        let mut merged = LinkedList::new();
        let mut ptr_a = list_a.start;
        let mut ptr_b = list_b.start;

        // 遍历两个链表，按序添加节点
        while let (Some(a), Some(b)) = (ptr_a, ptr_b) {
            let a_val = unsafe { &(*a.as_ptr()).val };
            let b_val = unsafe { &(*b.as_ptr()).val };

            if a_val <= b_val {
                // 取出a节点并添加到合并链表
                let next_a = unsafe { (*a.as_ptr()).next };
                merged.append_node(a);
                ptr_a = next_a;
            } else {
                // 取出b节点并添加到合并链表
                let next_b = unsafe { (*b.as_ptr()).next };
                merged.append_node(b);
                ptr_b = next_b;
            }
        }

        // 处理剩余节点（a链表）
        while let Some(a) = ptr_a {
            let next_a = unsafe { (*a.as_ptr()).next };
            merged.append_node(a);
            ptr_a = next_a;
        }

        // 处理剩余节点（b链表）
        while let Some(b) = ptr_b {
            let next_b = unsafe { (*b.as_ptr()).next };
            merged.append_node(b);
            ptr_b = next_b;
        }

        // 清空原链表的指针（避免析构时重复释放）
        list_a.start = None;
        list_a.end = None;
        list_a.length = 0;
        list_b.start = None;
        list_b.end = None;
        list_b.length = 0;

        merged
    }

    /// 辅助方法：直接添加节点指针到链表尾部（避免重新分配节点）
    fn append_node(&mut self, node: NonNull<Node<T>>) {
        unsafe { (*node.as_ptr()).next = None }; // 确保节点next为空
        match self.end {
            None => self.start = Some(node),
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = Some(node) },
        }
        self.end = Some(node);
        self.length += 1;
    }
}

// 实现Drop trait避免内存泄漏
impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.start;
        while let Some(node) = current {
            current = unsafe { (*node.as_ptr()).next };
            // 将指针转回Box并释放内存
            let _ = unsafe { Box::from_raw(node.as_ptr()) };
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
        while let Some(node) = current {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}", unsafe { &(*node.as_ptr()).val })?;
            current = unsafe { (*node.as_ptr()).next };
            first = false;
        }
        Ok(())
    }
}

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
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for &val in &vec_a {
            list_a.add(val);
        }
        for &val in &vec_b {
            list_b.add(val);
        }

        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);

        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for &val in &vec_a {
            list_a.add(val);
        }
        for &val in &vec_b {
            list_b.add(val);
        }

        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);

        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
}

use std::cmp::Ord;
use std::default::Default;

/// 通用二叉堆实现
/// - 支持最小堆/最大堆（通过比较函数控制）
/// - 底层用动态数组存储，索引从1开始（方便计算父子节点）
pub struct Heap<T>
where
    T: Default,
{
    count: usize,               // 堆中元素数量
    items: Vec<T>,              // 存储堆元素（索引0占位，从1开始）
    comparator: fn(&T, &T) -> bool, // 比较函数：最小堆(a < b)，最大堆(a > b)
}

impl<T> Heap<T>
where
    T: Default,
{
    /// 创建新堆
    /// comparator: 比较函数，返回true表示a应在b上方（堆顶方向）
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()], // 索引0占位
            comparator,
        }
    }

    /// 获取堆大小
    pub fn len(&self) -> usize {
        self.count
    }

    /// 判断堆是否为空
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 向堆中添加元素（上浮调整）
    pub fn add(&mut self, value: T) {
        // 1. 元素添加到数组末尾
        self.count += 1;
        self.items.push(value);
        // 2. 上浮调整：从最后一个元素开始，与父节点比较，不符合堆序则交换
        let mut idx = self.count;
        while idx > 1 {
            let parent_idx = self.parent_idx(idx);
            // 若当前元素应在父节点上方，则交换
            if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                self.items.swap(idx, parent_idx);
                idx = parent_idx;
            } else {
                break; // 堆序满足，停止调整
            }
        }
    }

    /// 获取父节点索引
    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    /// 判断当前节点是否有子节点
    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    /// 获取左子节点索引
    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    /// 获取右子节点索引
    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    /// 获取应上浮的子节点索引（用于下沉调整）
    /// - 最小堆：返回更小的子节点索引
    /// - 最大堆：返回更大的子节点索引
    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);

        // 只有左子节点
        if right_idx > self.count {
            return left_idx;
        }

        // 比较左右子节点，返回符合比较函数的那个
        if (self.comparator)(&self.items[left_idx], &self.items[right_idx]) {
            left_idx
        } else {
            right_idx
        }
    }

    /// 弹出堆顶元素（内部辅助方法）
    fn pop_top(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }

        // 1. 取出堆顶元素（索引1）
        let top = self.items.swap_remove(1);
        self.count -= 1;

        // 2. 下沉调整：将最后一个元素移到堆顶，逐步与子节点比较交换
        let mut idx = 1;
        while self.children_present(idx) {
            let child_idx = self.smallest_child_idx(idx);
            // 若当前节点不符合堆序，则与子节点交换
            if !(self.comparator)(&self.items[idx], &self.items[child_idx]) {
                self.items.swap(idx, child_idx);
                idx = child_idx;
            } else {
                break; // 堆序满足，停止调整
            }
        }

        Some(top)
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// 创建最小堆
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// 创建最大堆
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

/// 实现迭代器（依次弹出堆顶元素）
impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.pop_top()
    }
}

/// 最小堆构造器（语法糖）
pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

/// 最大堆构造器（语法糖）
pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(1));
        assert_eq!(heap.next(), None);
    }
}

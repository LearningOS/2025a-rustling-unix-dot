/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/
// I AM NOT DONE

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    /// 入队：添加元素到队列尾部
    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    /// 出队：移除并返回队列头部元素（空队列返回错误）
    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    /// 查看队列头部元素（不修改队列）
    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    /// 获取队列大小
    pub fn size(&self) -> usize {
        self.elements.len()
    }

    /// 判断队列是否为空
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

/// 用两个队列实现栈
/// 核心思路：
/// 1. q1 作为主队列存储栈元素，q2 作为临时队列
/// 2. push：先入队到q2，再将q1所有元素移到q2，最后交换q1/q2（保证新元素在q1头部）
/// 3. pop：直接出队q1（q1头部就是栈顶元素）
pub struct myStack<T> {
    q1: Queue<T>, // 主队列（存储栈元素，头部=栈顶）
    q2: Queue<T>, // 临时队列
}

impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new(),
        }
    }

    /// 入栈：将元素添加到栈顶
    pub fn push(&mut self, elem: T) {
        // 步骤1：新元素先入队到临时队列q2
        self.q2.enqueue(elem);

        // 步骤2：将q1所有元素依次移到q2（保证新元素在q2头部）
        while let Ok(val) = self.q1.dequeue() {
            self.q2.enqueue(val);
        }

        // 步骤3：交换q1和q2，让q1成为主队列（新元素在q1头部）
        std::mem::swap(&mut self.q1, &mut self.q2);
    }

    /// 出栈：移除并返回栈顶元素（空栈返回错误）
    pub fn pop(&mut self) -> Result<T, &str> {
        if self.q1.is_empty() {
            Err("Stack is empty")
        } else {
            self.q1.dequeue()
        }
    }

    /// 判断栈是否为空
    pub fn is_empty(&self) -> bool {
        self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut s = myStack::<i32>::new();
        // 空栈pop返回错误
        assert_eq!(s.pop(), Err("Stack is empty"));

        // 入栈 1,2,3
        s.push(1);
        s.push(2);
        s.push(3);

        // 出栈顺序：3 → 2
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));

        // 入栈 4,5
        s.push(4);
        s.push(5);

        // 栈非空
        assert_eq!(s.is_empty(), false);

        // 出栈顺序：5 →4 →1
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));

        // 空栈pop返回错误，且is_empty为true
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
    }
}

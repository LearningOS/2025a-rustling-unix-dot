/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
// I AM NOT DONE

fn sort<T: Ord>(array: &mut [T]) {
    // 空切片或单元素切片无需排序
    if array.len() <= 1 {
        return;
    }
    quick_sort(array, 0, array.len() - 1);
}

/// 快速排序核心实现（递归）
fn quick_sort<T: Ord>(array: &mut [T], low: usize, high: usize) {
    if low < high {
        // 分区：获取基准元素的最终位置
        let pivot_idx = partition(array, low, high);
        
        // 递归排序左半部分（注意处理pivot_idx=0的边界）
        if pivot_idx > 0 {
            quick_sort(array, low, pivot_idx - 1);
        }
        // 递归排序右半部分
        quick_sort(array, pivot_idx + 1, high);
    }
}

/// 分区函数：将数组分为两部分，左边<=基准，右边>=基准
fn partition<T: Ord>(array: &mut [T], low: usize, high: usize) -> usize {
    // 选择最右侧元素作为基准
    let pivot = high;
    let mut i = low;

    // 遍历[low, high-1]，将<=基准的元素移到左侧
    for j in low..high {
        if array[j] <= array[pivot] {
            array.swap(i, j);
            i += 1;
        }
    }

    // 将基准元素放到最终位置（i）
    array.swap(i, pivot);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }

    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }

    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }

    // 额外测试：空切片
    #[test]
    fn test_sort_empty() {
        let mut vec: Vec<i32> = vec![];
        sort(&mut vec);
        assert_eq!(vec, vec![]);
    }

    // 额外测试：字符串排序（验证泛型兼容性）
    #[test]
    fn test_sort_strings() {
        let mut vec = vec!["banana", "apple", "cherry", "date"];
        sort(&mut vec);
        assert_eq!(vec, vec!["apple", "banana", "cherry", "date"]);
    }
}

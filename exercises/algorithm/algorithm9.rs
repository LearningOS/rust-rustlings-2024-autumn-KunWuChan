/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;
use std::process::id;
use std::result;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,  // 存储
    comparator: fn(&T, &T) -> bool, // 重载运算符，堆大小比较规则
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    // 添加元素到堆中，并通过上浮（heapify-up）维护堆性质。
    pub fn add(&mut self, value: T) {
        self.count += 1;
        if self.count < self.items.len() {
            self.items[self.count] = value;
        } else {
            self.items.push(value);
        }
        
        let mut idx = self.count;
        while idx > 1 {
            let parent = self.parent_idx(idx);
            if ((self.comparator)(&self.items[idx], &self.items[parent])) {
                self.items.swap(idx, parent);
                idx = parent;
            } else {
                break;
            }
        }

    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    // 返回当前节点的子节点中“最小”（根据 comparator 定义）的索引。
    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);

        // 如果右子节点不存在或超出范围，返回左子节点
        if right > self.count {
            return left;
        }

        if ((self.comparator)(&self.items[left], &self.items[right])) {
            left
        } else {
            right
        }


    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap 最小堆，要求父节点最小
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        // 取出堆顶元素
        let result = self.items.swap_remove(1);
        self.count -= 1;

        if self.count == 0 {
            return Some(result);
        }

        // 将最后一个元素移到堆顶
        self.items[1] = self.items.pop().unwrap_or(T::default());

        let mut idx = 1;
        while self.children_present(idx) {
            let smallest = self.smallest_child_idx(idx);
            if ((self.comparator)(&self.items[smallest], &self.items[idx])){
                self.items.swap(idx, smallest);
                idx = smallest;
            }else {
                break;
            }
        }
        Some(result)

    }
}

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
    }
}
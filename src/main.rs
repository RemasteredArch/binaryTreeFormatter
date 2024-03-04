#![allow(dead_code)]

use std::fmt::{Display, Write};

use rand::Rng;

fn main() {
    let mut rand = rand::thread_rng();

    let mut heap: Heap<u32> = Heap::new();

    for _ in 1..=10 {
        heap.push(rand.gen_range(1..100))
    }

    println!("{}", heap);
}

/*struct MinHeap<T: Ord> {
    inner: Heap<T>,
}

impl<T: Ord> MinHeap<T> {}*/

struct Heap<T> {
    inner: Vec<T>,
}

impl<T> Heap<T> {
    fn new() -> Self {
        Heap {
            inner: vec![unsafe { std::mem::zeroed::<T>() }],
        }
    }

    fn len(&self) -> usize {
        self.inner.len() - 1
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn get(&self, index: usize) -> Option<&T> {
        assert!(index > 0);
        self.inner.get(index)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        assert!(index > 0);
        self.inner.get_mut(index)
    }
    fn get_parent(&self, index: usize) -> Option<&T> {
        self.get(Self::parent_index(index))
    }

    fn get_parent_mut(&mut self, index: usize) -> Option<&mut T> {
        self.get_mut(Self::parent_index(index))
    }

    fn get_left_child(&self, index: usize) -> Option<&T> {
        self.get(Self::left_child_index(index))
    }

    fn get_left_child_mut(&mut self, index: usize) -> Option<&mut T> {
        self.get_mut(Self::left_child_index(index))
    }

    fn get_right_child(&self, index: usize) -> Option<&T> {
        self.get(Self::right_child_index(index))
    }

    fn get_right_child_mut(&mut self, index: usize) -> Option<&mut T> {
        self.get_mut(Self::right_child_index(index))
    }

    fn get_last_node(&self) -> Option<&T> {
        self.get(self.last_node_index())
    }

    fn get_last_node_mut(&mut self) -> Option<&mut T> {
        self.get_mut(self.last_node_index())
    }

    fn push(&mut self, value: T) {
        self.inner.push(value)
    }

    fn take(&mut self) -> Option<T> {
        self.inner.pop()
    }

    fn swap(&mut self, a: usize, b: usize) {
        self.inner.swap(a, b);
    }

    fn parent_index(index: usize) -> usize {
        index / 2
    }

    fn left_child_index(index: usize) -> usize {
        index * 2
    }

    fn right_child_index(index: usize) -> usize {
        Self::left_child_index(index) + 1
    }

    fn last_node_index(&self) -> usize {
        self.len() - 1
    }
}

impl<T: Display> Display for Heap<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('[')?;
        for (index, value) in self.inner.iter().enumerate().skip(1) {
            value.fmt(f)?;
            if index <= self.last_node_index() {
                f.write_str(", ")?;
            }
        }
        f.write_char(']')
    }
}

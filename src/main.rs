#![allow(dead_code)]

use std::{
    fmt::{Display, Write},
    ops::{Deref, DerefMut},
    usize,
};

use rand::Rng;

fn main() {
    let mut rand = rand::thread_rng();

    let mut heap: MinHeap<u32> = MinHeap::new();

    for _ in 1..=10 {
        heap.push(rand.gen_range(1..100))
    }

    println!("{}", heap);
}

struct MinHeap<T: Ord> {
    inner: Heap<T>,
}

impl<T: Ord> Deref for MinHeap<T> {
    type Target = Heap<T>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Ord> DerefMut for MinHeap<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: Ord + Display> MinHeap<T> {
    fn new() -> Self {
        Self { inner: Heap::new() }
    }

    fn push(&mut self, value: T) {
        self.inner.push(value);
        self.bubble_up(self.last_node_index());
    }

    fn bubble_up(&mut self, index: usize) {
        if !Heap::<T>::has_parent(index) {
            return;
        }

        match (self.get_parent(index), self.get(index)) {
            (Some(parent), Some(child)) if parent > child => {
                let parent_index: usize = Heap::<T>::parent_index(index);
                self.swap(parent_index, index);
                self.bubble_up(parent_index);
            }

            _ => {}
        }
    }
}

impl<T: Display + Ord> Display for MinHeap<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

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
        self.inner.push(value);
    }

    fn take(&mut self) -> Option<T> {
        self.inner.pop()
    }

    fn swap(&mut self, a: usize, b: usize) {
        self.inner.swap(a, b);
    }

    fn last_node_index(&self) -> usize {
        self.len()
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

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn has_left_child(&self, index: usize) -> bool {
        Self::left_child_index(index) < self.inner.len()
    }

    fn has_right_child(&self, index: usize) -> bool {
        Self::right_child_index(index) < self.inner.len()
    }

    fn has_parent(index: usize) -> bool {
        index > 1
    }
}

impl<T: Display> Display for Heap<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('[')?;
        for (index, value) in self.inner.iter().enumerate().skip(1) {
            value.fmt(f)?;
            if index < self.last_node_index() {
                f.write_str(", ")?;
            }
        }
        f.write_char(']')
    }
}

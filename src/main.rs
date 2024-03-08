#![allow(dead_code)]

use std::{
    fmt::{Display, Write},
    ops::{Deref, DerefMut, RangeInclusive},
    usize,
};

use rand::{distributions::uniform::SampleUniform, Rng};

fn main() {
    let count: usize = 31;
    let max: u32 = 100;
    let heap: HeapPrinter<u32> = HeapPrinter::new_rand(count, 1..=max);

    println!("Heap ({}): {}\n", heap.len(), heap);

    heap.pretty_print();
}

struct HeapPrinter<T: Ord> {
    inner: MinHeap<T>,
    max_node_length: usize,
}

impl<T: Ord> Deref for HeapPrinter<T> {
    type Target = MinHeap<T>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Ord> DerefMut for HeapPrinter<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: Ord + Display> HeapPrinter<T> {
    fn new() -> Self {
        Self {
            inner: MinHeap::new(),
            max_node_length: 0,
        }
    }

    fn new_rand(count: usize, range: RangeInclusive<T>) -> Self
    where
        T: Clone + SampleUniform,
    {
        Self {
            inner: MinHeap::new_rand(count, range.clone()),
            max_node_length: range.end().to_string().len(),
        }
    }

    fn pretty_print(&self) {
        let count = self.len() - 1;
        let inverse_row_size: u32 = self.get_largest_row_size(count);
        self.print(
            1,
            1,
            self.max_node_length,
            inverse_row_size.try_into().unwrap(),
        );
    }

    fn print(
        &self,
        index: usize,
        row_size: usize,
        max_node_length: usize,
        inverse_row_size: usize,
    ) {
        if index >= self.len() {
            return;
        }

        let final_index: usize = index + row_size - 1;
        let node_length_padding: String = " ".repeat(max_node_length);
        let row_padding: String = node_length_padding.repeat(inverse_row_size - 1);
        self.print_row(index, final_index, row_padding, node_length_padding);
        self.print(
            final_index + 1,
            row_size * 2,
            max_node_length,
            inverse_row_size / 2,
        );
    }

    fn print_row(
        &self,
        index: usize,
        final_index: usize,
        row_padding: String,
        node_length_padding: String,
    ) {
        if let Some(node) = self.inner.get(index) {
            print!(
                "{0}{1:03$}{0}{2}",
                row_padding, node, node_length_padding, self.max_node_length
            );
        }

        if index == final_index {
            println!();
            return;
        }

        self.print_row(index + 1, final_index, row_padding, node_length_padding);
    }

    fn get_largest_row_size(&self, count: usize) -> u32 {
        let largest_bit_index = usize::BITS - count.leading_zeros(); // e.g. 20 -> 00010100
                                                                     //               ^ (5)
        2_u32.pow(largest_bit_index - 1) // 00010000 (index 5) -> 16
    }
}

impl<T: Ord + Display> Display for HeapPrinter<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
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

impl<T: Ord> MinHeap<T> {
    fn new() -> Self {
        Self { inner: Heap::new() }
    }

    fn new_rand(count: usize, range: RangeInclusive<T>) -> Self
    where
        T: Clone + SampleUniform,
    {
        let mut rand = rand::thread_rng();

        let mut new_heap = Self::new();

        for _ in 1..=count {
            new_heap.push(rand.gen_range(range.clone()));
        }

        new_heap
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

    fn to_padded_string(num: usize, width: usize, padding: char) -> String {
        let padded_length = width - num.to_string().len();
        padding.to_string().repeat(padded_length)
    }
}

impl<T: Ord + Display> Display for MinHeap<T> {
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
        self.inner.len()
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
        self.len() - 1
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
        self.len() == 1
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

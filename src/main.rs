/*
 * SPDX-License-Identifier: GPL-3.0-or-later
 *
 * Copyright © 2024 RemasteredArch
 *
 * This file is part of Binary Tree Formatter.
 *
 * Binary Tree Formatter is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 *
 * Binary Tree Formatter is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with Binary Tree Formatter. If not, see <https://www.gnu.org/licenses/>.
 *
 */

#![allow(dead_code)]

use std::{
    fmt::{Display, Write},
    ops::{Deref, DerefMut, RangeInclusive},
};

use clap::Parser;
use rand::{distributions::uniform::SampleUniform, Rng};

fn main() {
    let arguments = Arguments::parse();
    let heap: HeapPrinter<u32> = HeapPrinter::new_rand(arguments.nodes, 1..=arguments.range);

    println!("Heap ({}): {}\n", heap.len(), heap);

    heap.print();
}

#[derive(Parser)]
#[command(about, version, long_about = None)]
struct Arguments {
    /// Defines the number of nodes the heap
    #[arg(short = 'n', long = "nodes", default_value = "20")]
    nodes: usize,

    /// Defines the highest possible value for a node (nodes can range from [1..range])
    #[arg(short = 'r', long = "range", default_value = "50")]
    range: u32,
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
            max_node_length: range.end().to_string().len(),
            inner: MinHeap::new_rand(count, range),
        }
    }

    fn push(&mut self, value: T) {
        let length = value.to_string().len();

        self.max_node_length = self.max_node_length.max(length);

        self.inner.push(value);
    }

    fn print(&self) {
        let node_length_padding = " ".repeat(self.max_node_length);
        let count = self.len() - 1;

        let mut row_size: usize = 1;
        let mut index = row_size;
        let mut final_index = row_size;

        let mut inverse_row_size = self.get_largest_row_size(count);
        let mut row_padding = node_length_padding.repeat(inverse_row_size - 1);

        loop {
            self.print_row(index, final_index, &row_padding, &node_length_padding);

            index = final_index + 1;

            if index > self.last_node_index() {
                break;
            }

            row_size *= 2;
            final_index = index + row_size - 1;

            inverse_row_size /= 2;
            row_padding = node_length_padding.repeat(inverse_row_size - 1);
        }
    }

    fn print_row(
        &self,
        start_index: usize,
        mut final_index: usize,
        row_padding: &str,
        node_length_padding: &str,
    ) {
        // don't let it overflow
        final_index = final_index.min(self.last_node_index());

        for index in start_index..final_index {
            // print node with a zero-padded fixed-width and with spacing before and after
            print!(
                "{0}{1:03$}{0}{2}",
                row_padding,
                self.inner.get(index).unwrap(),
                node_length_padding,
                self.max_node_length
            );
        }

        // print last node with a line end and without spacing after
        println!(
            "{0}{1:02$}",
            row_padding,
            self.inner.get(final_index).unwrap(),
            self.max_node_length
        );
    }

    fn get_largest_row_size(&self, count: usize) -> usize {
        let largest_bit_index = usize::BITS - count.leading_zeros(); // e.g. 20 -> 00010100
                                                                     //               ^ (5)
        2_u32.pow(largest_bit_index - 1) as usize // 00010000 (index 5) -> 16
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

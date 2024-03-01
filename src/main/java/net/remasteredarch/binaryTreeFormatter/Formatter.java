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

package net.remasteredarch.binaryTreeFormatter;

import java.util.ArrayList;
import java.util.Random;
import java.util.function.Supplier;

public class Formatter {
	private static final int NODE_NUM_RANGE = 50; // max value used for nodes in the heap (0..(NODE_NUM_RANGE -1))
	private static final int MAX_NODE_LENGTH = (NODE_NUM_RANGE - 1 + "").length();
	private static final int NODE_COUNT = 20;

	public static void main(String[] args) {

		Supplier<Integer> rand = new RandomInteger(NODE_NUM_RANGE);
		MinHeap<Integer> heap = new MinHeap<>(NODE_NUM_RANGE, NODE_COUNT, rand);

		System.out.println("Heap: " + heap.toString());

		int splitIndex = 1;
		int rowSize = 1;
		System.out.print("* ");
		for (int heapIndex = 1; heapIndex < heap.size(); heapIndex++) {
			System.out.printf("%-" + MAX_NODE_LENGTH + "s ", heap.get(heapIndex));
			if (heapIndex == splitIndex) {
				System.out.print("\n* ");
				rowSize *= 2;
				splitIndex += rowSize;
			}
		}
		System.out.println();
	}
}

class RandomInteger implements Supplier<Integer> {
	private static Random rand = new Random();
	private int range;

	public RandomInteger(int range) {
		this.range = range;
	}

	@Override
	public Integer get() {
		return rand.nextInt(range);
	}
}

class MinHeap<N extends Number & Comparable<N>> extends Heap<N> {
	public MinHeap() {
	}

	public MinHeap(int valueRange, int values, Supplier<N> random) {
		for (int i = 0; i < values; i++) {
			add(random.get());
		}
	}

	@Override
	public void add(N value) {
		super.add(value);

		bubbleUp(lastNodeIndex());
	}

	protected void bubbleUp(int index) {
		if (!hasParent(index))
			return;

		if (parent(index).compareTo(get(index)) == 1) {
			swap(parentIndex(index), index);
			bubbleUp(parentIndex(index));
		}
	}
}

class Heap<T> {
	protected ArrayList<T> heap = new ArrayList<>();

	public Heap() {
		heap.add(null); // The first value of a Heap is index 1, index 0 doesn't matter
	}

	@Override
	public String toString() {
		return heap.toString();
	}

	public int size() {
		return heap.size();
	}

	public T get(int index) {
		return (T) heap.get(index);
	}

	public boolean isEmpty() {
		return heap.isEmpty();
	}

	public boolean hasParent(int index) {
		return index > 1;
	}

	protected T parent(int index) {
		return get(parentIndex(index));
	}

	protected int parentIndex(int index) {
		return index / 2;
	}

	protected boolean hasLeftChild(int index) {
		return leftChildIndex(index) < heap.size();
	}

	protected T leftChild(int index) {
		return get(leftChildIndex(index));
	}

	protected int leftChildIndex(int index) {
		return index * 2;
	}

	protected boolean hasRightChild(int index) {
		return rightChildIndex(index) < heap.size();
	}

	protected T rightChild(int index) {
		return get(rightChildIndex(index));
	}

	protected int rightChildIndex(int index) {
		return leftChildIndex(index) + 1;
	}

	protected T lastNode() {
		return get(lastNodeIndex());
	}

	protected int lastNodeIndex() {
		return heap.size() - 1;
	}

	protected T peekMin() {
		return get(1);
	}

	public void add(T value) {
		heap.add(value);
	}

	public T remove() {
		return (T) heap.remove(lastNodeIndex());
	}

	protected void swap(int first, int second) {
		T temp = get(first);
		heap.set(first, get(second));
		heap.set(second, temp);
	}
}

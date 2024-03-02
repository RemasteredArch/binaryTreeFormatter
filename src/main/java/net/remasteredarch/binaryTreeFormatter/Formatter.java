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
	private static final String PADDING = "  "; // between nodes in the tree

	private static final String RESET = "\033[0m";
	private static final String BOLD = "\033[1m";
	private static final String FAINT = "\033[90m"; // gray text

	public static void main(String[] args) {

		Supplier<Integer> rand = new RandomInteger(NODE_NUM_RANGE);
		MinHeap<Integer> heap = new MinHeap<>(NODE_NUM_RANGE, NODE_COUNT, rand);

		System.out.println(FAINT + BOLD + "Heap (" + heap.size() + "):" + RESET + FAINT + heap.toString() + RESET);

		System.out.print(BOLD + "\nTree:" + RESET);
		printTree(heap);
	}

	private static void printTree(MinHeap<Integer> heap) {
		int splitIndex = 1;
		int rowSize = 1;
		int indentSize = getMaxRowSize(NODE_COUNT);
		String padding = makePadding(indentSize);

		indent(rowSize, indentSize);

		for (int heapIndex = 1; heapIndex < heap.size(); heapIndex++) {
			System.out.printf("%s%0" + MAX_NODE_LENGTH + "d%s%s", padding, heap.get(heapIndex), padding, PADDING);

			if (heapIndex == splitIndex) {
				indentSize /= 2;
				rowSize *= 2;
				splitIndex += rowSize;
				padding = makePadding(indentSize);
				indent(rowSize, indentSize);
			}
		}

		System.out.println();
	}

	private static String makePadding(int rowSize) {
		String padding = "";

		for (int i = 0; i < rowSize - 1; i++) {
			padding += PADDING;
		}

		return padding;
	}

	private static int getMaxRowSize(int heapSize) {
		int count = 0;

		// there was logic when this was written.
		// however, i realize that i implemented the logic wrong.
		// i'm not quite sure why this works.
		while (heapSize > 1) {
			heapSize >>>= 1;
			count++;
		}

		int maxRowSize = (int) Math.pow(2, count);
		System.out.println(count + ": " + maxRowSize);

		return maxRowSize;
	}

	private static void indent(int rowSize, int indentSize) {
		System.out.printf("\n%-2s : %-2s * ", rowSize, indentSize);
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
		return heap.get(index);
	}

	public boolean isEmpty() {
		return heap.size() == 1;
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
		if (isEmpty())
			return null;

		return heap.remove(lastNodeIndex());
	}

	protected void swap(int first, int second) {
		T temp = get(first);
		heap.set(first, get(second));
		heap.set(second, temp);
	}
}

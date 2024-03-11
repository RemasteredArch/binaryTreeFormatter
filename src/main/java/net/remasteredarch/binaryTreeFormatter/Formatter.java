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
import java.util.function.Consumer;
import java.util.function.Supplier;

public class Formatter {
	private static int nodeNumRange = 51; // max value used for nodes in the heap (0..(NODE_NUM_RANGE - 1))
	private static int maxNodeLength = getMaxLength(nodeNumRange - 1);
	private static int maxRowSizeLength;
	private static int nodeCount = 20;
	private static Option[] options;

	private static final String NAME = "Binary Tree Formatter";
	private static final String VERSION = "v0.2";
	private static final String[] AUTHORS = { "2024 RemasteredArch" };
	private static final String PURPOSE = "Prints out a binary tree with formatting.";
	private static final String PATH = "src/main/java/net/remasteredarch/binaryTreeFormatter/Formatter.java";
	// private static final String PATH =
	// Formatter.class.getProtectionDomain().getCodeSource().getLocation().getPath();

	private static final char PADDING = ' '; // between nodes in the tree
	private static final String RESET = "\033[0m";
	private static final String BOLD = "\033[1m";
	private static final String FAINT = "\033[90m"; // gray text

	public static void main(String[] args) {
		parseOptions(args);

		Supplier<Integer> rand = new RandomInteger(nodeNumRange);
		MinHeap<Integer> heap = new MinHeap<>(nodeCount, rand);

		System.out.println(FAINT + BOLD + "Heap (" + heap.size() + "): " + RESET + FAINT + heap.toString() + RESET);

		System.out.println(BOLD + "\nTree:" + RESET);
		// printTree(heap);
		newPrintTree(heap);
	}

	private static void newPrintTree(Heap<Integer> heap) {
		final int lastNodeIndex = heap.lastNodeIndex();
		final String nodeLengthPadding = ("" + PADDING).repeat(maxNodeLength);

		int rowSize = 1;
		int index = rowSize;
		int finalIndex = rowSize;

		int inverseRowSize = getMaxRowSize(nodeCount);
		String rowPadding = nodeLengthPadding.repeat(inverseRowSize - 1);

		while (true) {
			printRow(index, finalIndex, rowPadding, nodeLengthPadding, lastNodeIndex, heap);

			index = finalIndex + 1;

			if (index > lastNodeIndex)
				break;

			rowSize *= 2;
			finalIndex = index + rowSize - 1;

			inverseRowSize /= 2;
			rowPadding = nodeLengthPadding.repeat(inverseRowSize - 1);
		}
	}

	private static void printRow(int startIndex, int finalIndex, String rowPadding, String nodeLengthPadding,
			int lastNodeIndex, Heap<Integer> heap) {
		// don't let it overflow
		finalIndex = Math.min(finalIndex, lastNodeIndex);

		for (int index = startIndex; index < finalIndex; index++) {
			// print a node with a zero-padded fixed-width and with spacing before and after
			System.out.printf("%s%0" + maxNodeLength + "d%1$s%3$s", rowPadding, heap.get(index), nodeLengthPadding);
		}

		// print last node with a line end and without spacing after
		System.out.printf("%s%0" + maxNodeLength + "d\n", rowPadding, heap.get(finalIndex));
	}

	private static void printTree(MinHeap<Integer> heap) {
		int splitIndex = 1;
		int rowSize = 1;
		int indentSize = getMaxRowSize(nodeCount);
		maxRowSizeLength = getMaxLength(indentSize);
		String nodeLengthPadding = makePadding(maxNodeLength, "" + PADDING);
		String padding = makePadding(indentSize - 1, nodeLengthPadding);

		indent(rowSize, indentSize);

		for (int heapIndex = 1; heapIndex < heap.size(); heapIndex++) {
			System.out.printf("%s%0" + maxNodeLength + "d%s%s", padding, heap.get(heapIndex), padding, nodeLengthPadding);

			if (heapIndex == splitIndex) {
				indentSize /= 2;
				if (indentSize == 0)
					break;
				rowSize *= 2;
				splitIndex += rowSize;
				padding = makePadding(indentSize - 1, nodeLengthPadding);
				indent(rowSize, indentSize);
			}
		}

		System.out.println();
	}

	private static String makePadding(int length, String basePadding) {
		return basePadding.repeat(length);
	}

	// The largest true bit in the binary representation of the size is the length
	// of the largest row. E.g. in 20 (110100), 010000 (16) is the largest true bit,
	// ignoring the sign bit. This bit shifts until the sign bit is the only true
	// bit left (heapSize > 1), at which point you'll know you've passed the real
	// largest true bit.
	//
	// Does Integer.highestOneBit(heapSize); work?
	private static int getMaxRowSize(int heapSize) {
		int count = 0;

		while (heapSize > 1) {
			heapSize >>>= 1;
			count++;
		}

		return (int) Math.pow(2, count);
	}

	private static void indent(int rowSize, int indentSize) {
		System.out.printf("\n%s%-" + maxRowSizeLength + "s%s : %s%-" + maxRowSizeLength + "s%s %s|%s ",
				FAINT, rowSize, RESET, FAINT, indentSize, RESET, BOLD, RESET);
	}

	private static int getMaxLength(int number) {
		return (number + "").length();
	}

	private static void parseOptions(String[] args) {
		options = loadOptions();

		argChecker: for (int i = 0; i < args.length; i++) {
			String arg = args[i];

			for (Option option : options) {
				if (option.longForm.equals("--help") && option.match(arg)) {
					option.action.accept(0);
				}

				if (option.match(arg)) {
					i++;
					try {
						option.action.accept(Integer.parseInt(args[i]));
					} catch (NumberFormatException e) {
						System.err.printf("%sError at \"%s\":%s Expected integer argument, received \"%s\".\n\n",
								BOLD, arg, RESET, args[i]);
						printHelpDialogue(1);
					}
					continue argChecker;
				}
			}

			System.err.println(BOLD + "Unrecognized Option \"" + RESET + arg + BOLD + "\"!\n" + RESET);
			printHelpDialogue(1);
		}
	}

	private static Option[] loadOptions() {
		Option help = new Option("--help", "-h",
				"Prints this help dialogue.",
				exitCode -> printHelpDialogue(exitCode));

		Option range = new Option("--range", "-r",
				"Sets the highest possible value for a node (nodes can range from [0..range]) (int, default: "
						+ (nodeNumRange - 1) + ").",
				value -> setRange(value + 1));

		Option nodes = new Option("--nodes", "-n",
				"Sets the number of nodes in the tree (int, default: " + nodeCount + ").",
				count -> setNodeCount(count));

		Option[] array = { help, range, nodes };

		Option.maxShortFormLength = 0;
		Option.maxLongFormLength = 0;

		for (Option option : array) {
			int shortFormLength = option.shortForm.length();
			int longFormLength = option.longForm.length();

			if (shortFormLength > Option.maxShortFormLength)
				Option.maxShortFormLength = shortFormLength;

			if (longFormLength > Option.maxLongFormLength)
				Option.maxLongFormLength = longFormLength;
		}

		return array;
	}

	private static void setRange(int range) {
		nodeNumRange = range;
		maxNodeLength = getMaxLength(nodeNumRange - 1);
	}

	private static void setNodeCount(int count) {
		nodeCount = count;
	}

	private static void printHelpDialogue(int exitCode) {
		HelpDialogueHelper helper = new HelpDialogueHelper(RESET, BOLD, FAINT);

		String dialogue = helper.title(NAME, VERSION, PURPOSE);
		for (String author : AUTHORS) {
			dialogue += helper.author(author);
		}

		dialogue += helper.section("Usage");
		dialogue += helper.item("$", "java", PATH, "[options]");

		dialogue += helper.section("Options");
		for (Option option : options) {
			dialogue += helper.item(option);
		}

		dialogue += helper.section("License");
		dialogue += helper.item(
				NAME + " is licensed under the GNU General Public License version 3, or (at your option) any later version.");
		dialogue += helper.item(
				"You should have received a copy of the GNU General Public License along with " + NAME
						+ ", found in LICENSE. If not, see <https://www.gnu.org/licenses/>.");

		if (exitCode != 0) {
			System.err.print(dialogue);
		} else {
			System.out.print(dialogue);
		}

		System.exit(exitCode);
	}
}

class HelpDialogueHelper {
	static final String INDENT = "  ";
	String reset;
	String bold;
	String faint;

	HelpDialogueHelper(String reset, String bold, String faint) {
		this.reset = reset;
		this.bold = bold;
		this.faint = faint;
	}

	String title(String title, String version, String purpose) {
		return String.format("%s%s (%s): %s%s\n", bold, title, version, reset, purpose);
	}

	String section(String title) {
		return String.format("\n%s%s%s:%s\n", reset, bold, title, reset);
	}

	String author(String author) {
		return String.format("%s%sCopyright © %s %s\n", INDENT, faint, author, reset);
	}

	String item(String prefix, String command, String path, String args) {
		return String.format("%s%s%s%s %s %s %s%s%s\n", INDENT, faint, prefix, reset, command, path, faint, args, reset);
	}

	String item(String item) {
		return String.format("%s%s%s%s\n", INDENT, faint, item, reset);
	}

	String item(Option option) {
		String format = String.format("%s%%-" + Option.maxShortFormLength + "s %s:%s %%s\t %s%%s%s\n",
				INDENT, faint, reset, faint, reset);

		return option.toString(format);
	}
}

class Option {
	String longForm; // e.g. --help
	static int maxLongFormLength;
	String shortForm; // e.g. -h
	static int maxShortFormLength;
	String purpose; // e.g. prints a help dialogue
	Consumer<Integer> action;

	public Option(String longForm, String shortForm, String purpose, Consumer<Integer> action) {
		this.longForm = longForm;
		this.shortForm = shortForm;
		this.purpose = purpose;
		this.action = action;
	}

	public boolean match(String arg) {
		return arg.equals(longForm) || arg.equals(shortForm);
	}

	@Override
	public String toString() {
		return toString("%-" + maxShortFormLength + "s | %-" + maxLongFormLength + "s\t %s");
	}

	// printf style formatting
	public String toString(String format) {
		return String.format(format, shortForm, longForm, purpose);
	}
}

class MinHeap<N extends Number & Comparable<N>> extends Heap<N> {
	public MinHeap() {
	}

	public MinHeap(int values, Supplier<N> random) {
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

		if (parent(index).compareTo(get(index)) > 0) {
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

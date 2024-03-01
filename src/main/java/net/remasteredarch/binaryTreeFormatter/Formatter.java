package net.remasteredarch.binaryTreeFormatter;

import java.util.ArrayList;

public class Formatter {
	static ArrayList<String> heap = new ArrayList<>();

	private static final int NODE_NUM_RANGE = 50; // max value used for nodes in the heap (0..(NODE_NUM_RANGE -1))
	private static final int MAX_NODE_LENGTH = ("" + (NODE_NUM_RANGE - 1)).length();

	public static void main(String[] args) {

		int splitIndex = 1;
		int rowSize = 1;
		for (int heapIndex = 1; heapIndex < heap.size(); heapIndex++) {
			System.out.printf("%" + MAX_NODE_LENGTH + "s ", heap.get(heapIndex));
			if (heapIndex == splitIndex) {
				System.out.println();
				rowSize *= 2;
				splitIndex += rowSize;
			}
		}
	}
}

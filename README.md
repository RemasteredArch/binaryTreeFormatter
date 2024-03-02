# Binary Tree Formatter

Prints out a binary tree with formatting.

Written and tested with OpenJDK 21 on Ubuntu 20.04 & 22.04. It may work on other versions, but there is no guarantee.

*Binary Tree Formatter is work in progress software. **Use at your own risk!***

## Usage

```
$ java src/main/java/net/remasteredarch/binaryTreeFormatter/Formatter.java
```

## Expected Output
As of commit [`ac23c4c`](https://github.com/RemasteredArch/binaryTreeFormatter/commit/ac23c4c2b9ab8387504bdb963350d6d0b8c1d108), it should output something similar to the following:
```
Heap: [null, 4, 4, 6, 15, 8, 11, 7, 29, 27, 8, 15, 44, 12, 38, 21, 49, 36, 32, 32, 41]

Tree:
* 4
* 4  6
* 15 8  11 7
* 29 27 8  15 44 12 38 21
* 49 36 32 32 41
```
*Note, however, that the true output contains font color/weight formatting and the values of the tree are random.*

## License

Binary Tree Formatter is licensed under the GNU General Public License version 3, or (at your option) any later version. You should have received a copy of the GNU General Public License along with Binary Tree Formatter, found in [LICENSE](./LICENSE). If not, see <[https://www.gnu.org/licenses/](https://www.gnu.org/licenses/)>.

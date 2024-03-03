# Binary Tree Formatter

Prints out a binary tree with formatting.

Written and tested with OpenJDK 21 on Ubuntu 20.04 & 22.04. It may work on other versions, but there is no guarantee.

*Binary Tree Formatter is work in progress software. **Use at your own risk!***

## Usage

```
$ java src/main/java/net/remasteredarch/binaryTreeFormatter/Formatter.java [options]
```
Options:
 * `-h`/`--help` Prints a help dialogue
 * `-r`/`--range` Sets the highest possible value for a node (nodes can range from [0..range]) (`int`, default: 50)
 * `-n`/`--nodes` Sets the number of nodes in the tree (`int`, default: 20)

## Expected Output

As of commit [`10aa374`](https://github.com/RemasteredArch/binaryTreeFormatter/tree/10aa374), it should output something similar to the following when not supplied with options:
```
Heap (21): [null, 2, 5, 15, 5, 17, 21, 18, 36, 7, 21, 43, 35, 42, 18, 40, 45, 40, 26, 15, 49]

Tree:
1  : 16 |                               02                                
2  : 8  |               05                              15                
4  : 4  |       05              17              21              18        
8  : 2  |   36      07      21      43      35      42      18      40    
16 : 1  | 45  40  26  15  49  
```
*Note, however, that the true output contains font color/weight formatting and the values of the tree are random.*

## License

Binary Tree Formatter is licensed under the GNU General Public License version 3, or (at your option) any later version. You should have received a copy of the GNU General Public License along with Binary Tree Formatter, found in [LICENSE](./LICENSE). If not, see <[https://www.gnu.org/licenses/](https://www.gnu.org/licenses/)>.

## Binary Trees

A binary tree is either empty or it is composed of a root element and two successors, which are binary trees themselves.

We shall use the following enum to represent binary trees (also available in [bintree/src/lib.rs](./bintree/src/lib.rs)). An End is equivalent to an empty tree. A Node has a value, and two descendant trees. To implement recursive data structure, we use `Box<T>` (For more details, see "[Using Box<T> to Point to Data on the Heap](https://doc.rust-lang.org/book/ch15-01-box.html)"). By implementing `fmt::Display` trait, you get a more compact output than automatically derived `fmt::Debug` implementation.

```rust
/// Binary Tree
#[derive(Debug, Clone, PartialEq)]
pub enum Tree<T: fmt::Display> {
    Node {
        value: T,
        left: Box<Tree<T>>,
        right: Box<Tree<T>>,
    },
    End,
}

impl<T: fmt::Display> fmt::Display for Tree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Tree::Node { value, left, right } => write!(f, "T({} {} {})", value, left, right),
            Tree::End => write!(f, "."),
        }
    }
}
```

![](./images/bintree.gif)

The example tree on the above is given by:

(Here, `node()`, `leaf()`, and `end()` are associated functions that construct Tree instances. see [bintree/src/lib.rs](./bintree/src/lib.rs) for details.)

```rust
let tree = Tree::node(
    'a',
    Tree::node('b', Tree::leaf('d'), Tree::leaf('e')),
    Tree::node(
        'c',
        Tree::end(),
        Tree::node('f', Tree::leaf('g'), Tree::end()),
    ),
);
```

### P54 (*) Check whether a given term represents a binary tree

Omitted; our tree representation will only allow well-formed trees.

### [P55](./P55/src/lib.rs) (**) Construct completely balanced binary trees.

In a completely balanced binary tree, the following property holds for every node: The number of nodes in its left subtree and the number of nodes in its right subtree are almost equal, which means their difference is not greater than one. 

Write a function `cbal_trees()` to construct completely balanced binary trees for a given number of nodes. The function should generate all solutions. The function should take as parameters the number of nodes and a single value to put in all of them.

Example: [examples/cbal_trees.rs](./P55/examples/cbal_trees.rs)
```rust
let trees = cbal_tree(4, 'x');
for tree in trees {
    println!("{}", tree);
}
```

```bash
P55 $ cargo run -q --example cbal_trees
T(x T(x . .) T(x . T(x . .)))
T(x T(x . T(x . .)) T(x . .))
T(x T(x . .) T(x T(x . .) .))
T(x T(x T(x . .) .) T(x . .))
```

### [P56](./P56/src/lib.rs) (**) Symmetric binary trees.

Let us call a binary tree symmetric if you can draw a vertical line through the root node and then the right subtree is the mirror image of the left subtree. Write a function `is_symmetric()` to check whether a given binary tree is symmetric. 

Hint: Write an `is_mirror_of()` function first to check whether one tree is the mirror image of another. We are only interested in the structure, not in the contents of the nodes. 

Example: [examples/is_symmetric.rs](./P56/examples/is_symmetric.rs)
```rust
let tree = Tree::node('a', Tree::leaf('b'), Tree::leaf('c'));
println!(
    "{} is {}.",
    tree,
    if is_symmetric(&tree) { "symmetric" } else { "not symmetric" }
);

let tree = Tree::node(
    'a',
    Tree::node('b', Tree::leaf('d'), Tree::end()),
    Tree::node('c', Tree::leaf('e'), Tree::end()),
);
println!(
    "{} is {}.",
    tree,
    if is_symmetric(&tree) { "symmetric" } else { "not symmetric" }
);
```

```bash
P56 $ cargo run -q --example is_symmetric
T(a T(b . .) T(c . .)) is symmetric.
T(a T(b T(d . .) .) T(c T(e . .) .)) is not symmetric.
```

### [P57](./P57/src/lib.rs) (**) Binary search trees (dictionaries).

Write a function to add an element to a binary search tree.

Example: [examples/binsearch.rs](./P57/examples/binsearch.rs)
```rust
let mut tree = Tree::end();
add_value(&mut tree, 2);
println!("add 2: {}", tree);

add_value(&mut tree, 3);
println!("add 3: {}", tree);

add_value(&mut tree, 0);
println!("add 0: {}", tree);
```

```bash
P57 $ cargo run -q --example binsearch
add 2: T(2 . .)
add 3: T(2 . T(3 . .))
add 0: T(2 T(0 . .) T(3 . .))
```

Use that function to construct a binary tree from a list of integers.

Finally, use that function to test your solution to P56.

Example: [examples/from_list.rs](./P57/examples/from_list.rs)
```rust
let values = vec![3, 2, 5, 7, 1];
let tree = from_list(&values);
println!("from {:?}: {}", values, tree);

println!("----");

let values = vec![5, 3, 18, 1, 4, 12, 21];
let tree = from_list(&values);
println!("from_list({:?}) is {}.", values, if is_symmetric(&tree) { "symmetric" } else { "not symmetric" });

let values = vec![3, 2, 5, 7, 4];
let tree = from_list(&values);
println!("from_list({:?}) is {}.", values, if is_symmetric(&tree) { "symmetric" } else { "not symmetric" });
```

```bash
P57 $ cargo run -q --example from_list
from [3, 2, 5, 7, 1]: T(3 T(2 T(1 . .) .) T(5 . T(7 . .)))
----
from_list([5, 3, 18, 1, 4, 12, 21]) is symmetric.
from_list([3, 2, 5, 7, 4]) is not symmetric.
```

### [P58](./P58/src/lib.rs) (**) Generate-and-test paradigm.

Apply the generate-and-test paradigm to construct all symmetric, completely balanced binary trees with a given number of nodes.

Example [examples/symmetric_balanced_trees.rs](./P58/examples/symmetric_balanced_trees.rs)
```rust
let trees = symmetric_balanced_trees(5, 'x');
for tree in trees {
    println!("{}", tree);
}
```

```bash
P58 $ cargo run -q --example symmetric_balanced_trees
T(x T(x . T(x . .)) T(x T(x . .) .))
T(x T(x T(x . .) .) T(x . T(x . .)))
```

### [P59](./P59/src/lib.rs) (**) Construct height-balanced binary trees.

In a height-balanced binary tree, the following property holds for every node: The height of its left subtree and the height of its right subtree are almost equal, which means their difference is not greater than one. 

Write a function `hbal_trees()` to construct height-balanced binary trees for a given height with a supplied value for the nodes. The function should generate all solutions.

Example [examples/hbal_tree.rs](./P59/examples/hbal_trees.rs)
```rust
let trees = hbal_trees(3, 'x');
for tree in trees {
    println!("{}", tree);
}
```

```bash
P59 $ cargo run -q --example hbal_trees
T(x T(x T(x . .) T(x . .)) T(x T(x . .) T(x . .)))
T(x T(x T(x . .) T(x . .)) T(x T(x . .) .))
T(x T(x T(x . .) T(x . .)) T(x . T(x . .)))
T(x T(x T(x . .) .) T(x T(x . .) T(x . .)))
T(x T(x T(x . .) .) T(x T(x . .) .))
T(x T(x T(x . .) .) T(x . T(x . .)))
T(x T(x . T(x . .)) T(x T(x . .) T(x . .)))
T(x T(x . T(x . .)) T(x T(x . .) .))
T(x T(x . T(x . .)) T(x . T(x . .)))
T(x T(x T(x . .) T(x . .)) T(x . .))
T(x T(x . .) T(x T(x . .) T(x . .)))
T(x T(x T(x . .) .) T(x . .))
T(x T(x . .) T(x T(x . .) .))
T(x T(x . T(x . .)) T(x . .))
T(x T(x . .) T(x . T(x . .)))
```

### [P60](./P60/src/lib.rs) (**) Construct height-balanced binary trees with a given number of nodes.

Consider a height-balanced binary tree of height _h_. What is the maximum number of nodes it can contain? Clearly, MaxN = 2^h - 1. However, what is the minimum number MinN? This question is more difficult. Try to find a recursive statement and turn it into a function `min_hbal_nodes()` that takes a height and returns MinN.

Example: [examples/min_hbal_nodes.rs](./P60/examples/min_hbal_nodes.rs)
```rust
println!("{}", min_hbal_nodes(3));
```

```bash
P60 $ cargo run -q --example min_hbal_nodes
4
```

On the other hand, we might ask: what is the maximum height _h_ a height-balanced binary tree with _n_ nodes can have? Write a `max_hbal_height()` function.

Example: [examples/max_hbal_height.rs](./P60/examples/max_hbal_height.rs)
```rust
println!("{}", max_hbal_height(4));
```

```bash
P60 $ cargo run -q --example max_hbal_height
3
```

Now, we can attack the main problem: construct all the height-balanced binary trees with a given nuber of nodes.

Example: [examples/hbal_trees_with_nodes.rs](./P60/examples/hbal_trees_with_nodes.rs)
```rust
let hbal_trees = hbal_trees_with_nodes(4, 'x');
for tree in hbal_trees {
    println!("{}", tree)
}
```

```bash
P60 $ cargo run -q --example hbal_trees_with_nodes
T(x T(x T(x . .) .) T(x . .))
T(x T(x . .) T(x T(x . .) .))
T(x T(x . T(x . .)) T(x . .))
T(x T(x . .) T(x . T(x . .)))
```

### [P61](./P61/src/lib.rs) (*) Count the leaves of a binary tree.

A leaf is a node with no successors. Write a method `leaf_cunt()` to count them.

Example: [examples/leaf_count.rs](./P61/examples/leaf_count.rs)
```rust
let tree = Tree::node('x', Tree::leaf('x'), Tree::end());
println!("{}", leaf_count(&tree));
```

```bash
P61 $ cargo run -q --example leaf_count
1
```

### [P61A](./P61A/src/lib.rs) (*) Collect the leaves of a binary tree in a list.

A leaf is a node with no successors. Write a function `leaf_list()` to collect them in a list. 

Example: [examples/leaf_list.rs](./P61A/examples/leaf_list.rs)
```rust
let tree = Tree::node(
    'a',
    Tree::leaf('b'),
    Tree::node('c', Tree::leaf('d'), Tree::leaf('e')),
);
println!("{:?}", leaf_list(&tree));
```

```bash
P61A $ cargo run -q --example leaf_list
['b', 'd', 'e']
```

### [P62](./P62/src/lib.rs) (*) Collect the internal nodes of a binary tree in a list.

An internal node of a binary tree has either one or two non-empty successors. Write a function `internal_list()` to collect them in a list. 

Example: [examples/internal_list.rs](./P62/examples/internal_list.rs)
```rust
let tree = Tree::node(
    'a',
    Tree::leaf('b'),
    Tree::node('c', Tree::leaf('d'), Tree::leaf('e')),
);
println!("{:?}", internal_list(&tree));
```

```bash
P62 $ cargo run -q --example internal_list
['a', 'c']
```

### [P62B](./P62B/src/lib.rs) (*) Collect the nodes at a given level in a list.

A node of a binary tree is at level N if the path from the root to the node has length N-1. The root node is at level 1. Write a function `at_level()` to collect all nodes at a given level in a list.

Example: [examples/at_level.rs](./P62B/examples/at_level.rs)
```rust
let tree = Tree::node(
    'a',
    Tree::leaf('b'),
    Tree::node('c', Tree::leaf('d'), Tree::leaf('e')),
);
println!("{:?}", at_level(&tree, 2));
```

```bash
P62B $ cargo run -q --example at_level
['b', 'c']
```

### [P63](./P63/src/lib.rs) (**) Construct a complete binary tree.

A complete binary tree with height _H_ is defined as follows: The levels 1,2,3,...,_H_-1 contain the maximum number of nodes (i.e 2^(i-1) at the level i, note that we start counting the levels from 1 at the root). In level _H_, which may contain less than the maximum possible number of nodes, all the nodes are "left-adjusted". This means that in a levelorder tree traversal all internal nodes come first, the leaves come second, and empty successors (the Ends which are not really nodes!) come last.

Particularly, complete binary trees are used as data structures (or addressing schemes) for heaps.

We can assign an address number to each node in a complete binary tree by enumerating the nodes in levelorder, starting at the root with number 1. In doing so, we realize that for every node _X_ with address _A_ the following property holds: The address of _X_'s left and right successors are 2 \* _A_ and 2 \* _A_+1, respectively, supposed the successors do exist. This fact can be used to elegantly construct a complete binary tree structure. Write a function `complete_binary_tree()` that takes as parameters the number of nodes and the value to put in each node.

Example: [examples/cbt.rs](./P63/examples/cbt.rs)
```rust
let cbt = complete_binary_tree(6, 'x');
println!("{}", cbt);
```

```bash
P63 $ cargo run -q --example cbt
T(x T(x T(x . .) T(x . .)) T(x T(x . .) .))
```

### [P64](./P64/src/lib.rs) (**) Layout a binary tree (1).

As a preparation for drawing a tree, a layout algorithm is required to determine the position of each node in a rectangular grid. Several layout methods are conceivable, one of them is shown in the illustration on the below.

![](./images/p64.gif)

In this layout strategy, the position of a node v is obtained by the following two rules:

- x(v) is equal to the position of the node v in the inorder sequence
- y(v) is equal to the depth of the node v in the tree

In order to store the position of the nodes, we add a new struct with the additional information (also available in [bintree_pos/src/lib.rs](./bintree_pos/src/lib.rs)).

```rust
/// Positioned Tree; a Tree with position (x, y)
#[derive(Debug, Clone, PartialEq)]
pub enum PositionedTree<T: fmt::Display> {
    Node {
        value: T,
        left: Box<PositionedTree<T>>,
        right: Box<PositionedTree<T>>,
        x: i32,
        y: i32,
    },
    End,
}

impl<T: fmt::Display> fmt::Display for PositionedTree<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            PositionedTree::Node {
                value,
                left,
                right,
                x,
                y,
            } => write!(f, "T[{},{}]({} {} {})", x, y, value, left, right),
            PositionedTree::End => write!(f, "."),
        }
    }
}
```

Write a function `layout_bintree()` that turns a normal Tree into a PositionedTree.

Example: [examples/layout_bintree.rs](./P64/examples/layout_bintree.rs)
```rust
let tree = Tree::node(
    'a',
    Tree::node('b', Tree::end(), Tree::leaf('c')),
    Tree::leaf('d'),
);
println!("{}", layout_bintree(&tree))
```

```bash
P64 $ cargo run -q --example layout_bintree
T[3,1](a T[1,2](b . T[2,3](c . .)) T[4,2](d . .))
```

The tree in the above illustration may be constructed with `from_list(&vec!['n', 'k', 'm', 'c', 'a', 'h', 'g', 'e', 'u', 'p', 's', 'q'])` (defined on P57). Use it to check your code.

### [P65](./P65/src/lib.rs) (**) Layout a binary tree (2).

An alternative layout method is depicted in the illustration below. Find out the rules and write the corresponding method. 

Hint: On a given level, the horizontal distance between neighboring nodes is constant. 

![](./images/p65.gif)

Use the same conventions as in problem P64.

Example: [examples/layout_bintree2.rs](./P65/examples/layout_bintree2.rs)
```rust
let tree = Tree::node(
    'a',
    Tree::node('b', Tree::end(), Tree::leaf('c')),
    Tree::leaf('d'),
);
println!("{}", layout_bintree2(&tree))
```

```bash
P65 $ cargo run -q --example layout_bintree2
T[3,1](a T[1,2](b . T[2,3](c . .)) T[5,2](d . .))
```

### P66 (***) Layout a binary tree (3).

Yet another layout strategy is shown in the illustration below. The method yields a very compact layout while maintaining a certain symmetry in every node. Find out the rules and write the corresponding method. 

Hint: Consider the horizontal distance between a node and its successor nodes. How tight can you pack together two subtrees to construct the combined binary tree?

![](./images/p66.gif)

### [P67](./P67/src/lib.rs) (**) A string representation of binary trees.

Somebody represents binary trees as strings of the following type (see example below):

```
a(b(d,e),c(,f(g,)))
```

![](./images/bintree.gif)

a) Implement `std::fmt::Display` for the Tree struct below to generate its string representation, if the tree is given as usual (in Nodes and Ends). 

For simplicity, suppose the information in the nodes is a single letter and there are no spaces in the string.

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Tree {
    Node {
        value: char,
        left: Box<Tree>,
        right: Box<Tree>,
    },
    End,
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // FILL THIS METHOD
    }
}
```

Hint: You can find how does the method look like on ["Rust by Example - Display"](https://doc.rust-lang.org/stable/rust-by-example/hello/print/print_display.html).

Example: [examples/tree_to_string.rs](./P67/examples/tree_to_string.rs)
```rust
let tree = Tree::node(
    'a',
    Tree::node('b', Tree::leaf('d'), Tree::leaf('e')),
    Tree::node(
        'c',
        Tree::end(),
        Tree::node('f', Tree::leaf('g'), Tree::end()),
    ),
);
println!("{}", tree);
```

```bash
P67 $ cargo run -q --example tree_to_string
a(b(d,e),c(,f(g,)))
```

(Here, `node()`, `leaf()`, and `end()` are associated functions that construct Tree instances.)

b) Write a function which does this inverse; i.e. given the string representation, construct the tree in the usual form.

Example: [examples/tree_from_string.rs](./P67/examples/tree_from_string.rs)
```rust
let tree = Tree::from_string("a(b(d,e),c(,f(g,)))");
println!("{}", tree);
```

```bash
P67 $ cargo run -q --example tree_from_string
a(b(d,e),c(,f(g,)))
```

### [P68](./P68/src/lib.rs) (**) Preorder and inorder sequences of binary trees.

We consider binary trees with nodes that are identified by single lower-case letters, as in the example of problem P67.

a) Write functions `preorder()` and `inorder()` that construct the preorder and inorder sequence of a given binary tree, respectively. The results should be vectors, e.g. `vec!['a','b','d','e','c','f','g']` for the preorder sequence of the example in problem P67.

Example: [examples/preorder.rs](./P68/examples/preorder.rs)
```rust
let tree = Tree::from_string("a(b(d,e),c(,f(g,)))");
println!("{:?}", preorder(&tree));
```

```bash
P68 $ cargo run -q --example preorder
['a', 'b', 'd', 'e', 'c', 'f', 'g']
```

Example: [examples/inorder.rs](./P68/examples/inorder.rs)
```rust
let tree = Tree::from_string("a(b(d,e),c(,f(g,)))");
println!("{:?}", inorder(&tree));
```

```bash
P68 $ cargo run -q --example inorder
['d', 'b', 'e', 'a', 'c', 'g', 'f']
```

b) Can you add an inverse functions of problem a); i.e. given a preorder sequence (a vector), construct a corresponding tree? If not, make the necessary arrangements.

Hint: You may want to change the Tree implementation we defined on P67 to modify the existing tree structure. (Say, add `replace_left()` and/or `replace_right()`.)

Example: [examples/from_preorder.rs](./P68/examples/from_preorder.rs)
```rust
let trees = from_preorder(&vec!['a', 'b', 'c']);
for tree in trees {
    println!("{}", tree);
}
```

```bash
P68 $ cargo run -q --example from_preorder
a(b(c,),)
a(b(,c),)
a(b,c)
a(,b(c,))
a(,b(,c))
```

Example: [examples/from_inorder.rs](./P68/examples/from_inorder.rs)
```rust
let trees = from_inorder(&vec!['a', 'b', 'c']);
for tree in trees {
    println!("{}", tree);
}
```

```bash
P68 $ cargo run -q --example from_inorder
c(b(a,),)
b(a,c)
c(a(,b),)
a(,b(,c))
a(,c(b,))
```

c) If both the preorder sequence and the inorder sequence of the nodes of a binary tree are given, then the tree is determined unambiguously. Write a function `pre_in_tree()` that does the job.

Example: [examples/pre_in_tree.rs](./P68/examples/pre_in_tree.rs)
```rust
let trees = pre_in_tree(
    &vec!['a', 'b', 'd', 'e', 'c', 'f', 'g'],
    &vec!['d', 'b', 'e', 'a', 'c', 'g', 'f'],
);
println!("{}", trees[0]);
```

```bash
P68 $ cargo run -q --example pre_in_tree
a(b(d,e),c(,f(g,)))
```

What happens if the same character appears in more than one node? Try, for instance, `pre_in_tree(&vec!['a', 'b', 'a'], vec!['b', 'a', 'a'])`.

### [P69](./P69/src/lib.rs) (**) Dotstring representation of binary trees.

We consider again binary trees with nodes that are identified by single lower-case letters, as in the example of problem P67. Such a tree can be represented by the preorder sequence of its nodes in which dots (.) are inserted where an empty subtree (End) is encountered during the tree traversal. For example, the tree shown in problem P67 is represented as "abd..e..c.fg...". First, try to establish a syntax (BNF or syntax diagrams) and then write two functions, `to_dotstring()` and `from_dotstring()`, which do the conversion in both directions.

Example: [examples/to_dotstring.rs](./P69/examples/to_dotstring.rs)
```rust
let dstr = to_dotstring(&Tree::from_string("a(b(d,e),c(,f(g,)))"));
println!("{}", dstr);
```

```bash
P69 $ cargo run -q --example to_dotstring
abd..e..c.fg...
```

Example: [examples/from_dotstring.rs](./P69/examples/from_dotstring.rs)
```rust
let tree = from_dotstring("abd..e..c.fg...");
println!("{}", tree);
```

```bash
P69 $ cargo run -q --example from_dotstring
a(b(d,e),c(,f(g,)))
```
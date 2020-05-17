## Multiway Trees

A multiway tree is composed of a root element and a (possibly empty) set of successors which are multiway trees themselves. A multiway tree is never empty. The set of successor trees is sometimes called a forest.

![](./images/p70.gif)

The code to represent these (also available in [mtree/src/lib.rs](./mtree/src/lib.rs)) is somewhat simpler than the code for binary trees, partly because we don't separate classes for nodes and terminators, and partly because we don't need the restriction that the value type be ordered.

```rust
/// Multiway Tree
pub struct MTree {
    value: char,
    children: Vec<MTree>,
}

impl MTree {
    /// Creates a leaf node
    pub fn leaf(value: char) -> Self {
        MTree::node(value, vec![])
    }
    /// Creates a node with children
    pub fn node(value: char, children: Vec<MTree>) -> Self {
        MTree {
            value: value,
            children: vec![],
        }
    }
}
```

The example tree is, thus:

```rust
MTree::node(
    'a',
    vec![
        MTree::node('f', vec![MTree::leaf('g')]),
        MTree::leaf('c'),
        MTree::node('b', vec![MTree::leaf('d'), MTree::leaf('e')])
    ]
)
```

### P70B (*) Check whether a given term represents a multiway tree

Omitted; our tree representation will only allow well-formed trees.

### [P70C](./P70C/src/lib.rs) (*) Count the nodes of a multiway tree.

Write a function `node_count()` which counts the nodes of a given multiway tree.

Example: [examples/node_count.rs](./P70C/examples/node_count.rs)
```rust
let tree = MTree::node('a', vec![MTree::leaf('f')]);
println!("node count = {}", node_count(&tree));
```

```bash
P70C $ cargo run -q --example node_count
node count = 2
```

### [P70](./P70/src/lib.rs) (**) Tree construction from a node string.

We suppose that the nodes of a multiway tree contain single characters. In the depth-first order sequence of its nodes, a special character ^ has been inserted whenever, during the tree traversal, the move is a backtrack to the previous level.

![](./images/p70.gif)

By this rule, the tree in the figure above is represented as:

```
afg^^c^bd^e^^^
```

Define the syntax of the string and write a function `str_to_tree()` to construct an MTree from a String. Also write the reverse function `tree_to_str()`.

Example: [examples/str_to_tree.rs](./P70/examples/str_to_tree.rs)
```rust
let s = "afg^^c^bd^e^^^";
println!("{:?}", str_to_tree(s));
```

```bash
P70 $ cargo run -q --example str_to_tree
MTree { value: 'a', children: [MTree { value: 'f', children: [MTree { value: 'g', children: [] }] }, MTree { value: 'c', children: [] }, MTree { value: 'b', children: [MTree { value: 'd', children: [] }, MTree { value: 'e', children: [] }] }] }
```

Example: [examples/tree_to_str.rs](./P70/examples/tree_to_str.rs)
```rust
let tree = MTree::node(
    'a',
    vec![
        MTree::node('f', vec![MTree::leaf('g')]),
        MTree::leaf('c'),
        MTree::node('b', vec![MTree::leaf('d'), MTree::leaf('e')]),
   ],
);
println!("{}", tree_to_str(&tree));
```

```bash
P70 $ cargo run -q --example tree_to_str
afg^^c^bd^e^^^
```

### [P71](./P71/src/lib.rs) (*) Determine the internal path length of a tree.

We define the internal path length of a multiway tree as the total sum of the path lengths from the root to all nodes of the tree. By this definition, the tree in the figure of problem P70 has an internal path length of 9. Write a function `internal_path_length()` to return that sum.

Example: [examples/internal_path_length.rs](./P71/examples/internal_path_length.rs)
```rust
println!("{}", internal_path_length(&str_to_tree("afg^^c^bd^e^^^")));
```

```bash
P71 $ cargo run -q --example internal_path_length
9
```

### [P72](./P72/src/lib.rs) (*) Construct the postorder sequence of the tree nodes.

Write a function `postorder()` which constructs the postorder sequence of the nodes of a multiway tree. The result should be a vector.

Example: [examples/postorder.rs](./P72/examples/postorder.rs)
```rust
let tree = str_to_tree("afg^^c^bd^e^^^");
println!("{:?}", postorder(&tree));
```

```bash
P72 $ cargo run -q --example postorder
['g', 'f', 'c', 'd', 'e', 'b', 'a']
```

### [P73](./P73/src/lib.rs) (**) Lisp-like tree representation.

There is a particular notation for multiway trees in Lisp. Lisp is a prominent functional programming language. In Lisp almost everything is a list.

Our example tree would be represented in Lisp as (a (f g) c (b d e)). The following pictures give some more examples.

![](./images/p73.png)

Note that in the "lispy" notation a node with successors (children) in the tree is always the first element in a list, followed by its children. The "lispy" representation of a multiway tree is a sequence of atoms and parentheses '(' and ')', with the atoms separated by spaces. We can represent this syntax as a string. Write a function `lispy_tree()` which constructs a "lispy string" from an MTree.

Example: [examples/lispy_tree.rs](./P73/examples/lispy_tree.rs)
```rust
let tree = MTree::node(
    'a',
    vec![
        MTree::node('f', vec![MTree::leaf('g')]),
        MTree::leaf('c'),
        MTree::node('b', vec![MTree::leaf('d'), MTree::leaf('e')]),
    ],
);
println!("{}", lispy_tree(&tree));
```

```bash
P73 $ cargo run -q --example lispy_tree
(a (f g) c (b d e))
```

As a second, even more interesting, exercise try to write a function that takes a "lispy" string and turns it into a multiway tree.

Example: [examples/lispy_str_to_tree.rs](./P73/examples/lispy_str_to_tree.rs)
```rust
let s = "(a (f g) c (b d e))";
println!("{:?}", lispy_str_to_tree(&s));
```

```bash
P73 $ cargo run -q --example lispy_str_to_tree
MTree { value: 'a', children: [MTree { value: 'f', children: [MTree { value: 'g', children: [] }] }, MTree { value: 'c', children: [] }, MTree { value: 'b', children: [MTree { value: 'd', children: [] }, MTree { value: 'e', children: [] }] }] }
```
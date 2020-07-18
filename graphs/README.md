## Graphs

A graph is defined as a set of nodes and a set of edges, where each edge is a pair of nodes.

![](./images/graph1.gif)

There are several ways to represent graphs. According to the definition of the graph as a pair of two sets (nodes and edges), we may use the following representation for the example graph:

```rust
let graph = from_term_form(
    &vec!['b', 'c', 'd', 'f', 'g', 'h', 'k'],
    &vec![('b', 'c'), ('b', 'f'), ('c', 'f'), ('f', 'k'), ('g', 'h')],
);
```

We call this _graph-term_ form. Note, that the lists are kept sorted, they are really _sets_, without duplicated elements. Each edge appears only once in the edge list; i.e. an edge from a node x to another node y is represented as `(x, y)`, the tuple `(y, x)` is not present.

Another representation method is to associate with each node the set of nodes that are adjacent to that node. We call this the _adjacency-list_ form. In our example: 

```rust
let graph = from_adjacent_form(&vec![
    ('b', &vec!['c', 'f']), ('c', &vec!['b', 'f']), ('d', &vec![]),
    ('f', &vec!['b', 'c', 'k']), ('g', &vec!['h']), ('h', &vec!['g']), ('k', &vec!['f']),
]);
```

In an undirected graph, care must be taken to ensure that all links are symmetric -- if _b_ is adjacent to _c_, _c_ must also be adjacent to _b_.

The representations we introduced so far are bound to our implementation and therefore well suited for automated processing, but their syntax is not very user-friendly. Typing the terms by hand is cumbersome and error-prone. We can define a more compact and "human-friendly" notation as follows: A graph is represented by a string of terms of the type X or Y-Z separated by commas. The standalone terms stand for isolated nodes, the Y-Z terms describe edges. If an X appears as an endpoint of an edge, it is automatically defined as a node. Our example could be written as:

```
[b-c, f-c, g-h, d, f-b, k-f, h-g]
```

We call this the _human-friendly_ form. As the example shows, the list does not have to be sorted and may even contain the same edge multiple times. Notice the isolated node _d_.

When the edges are _directed_, we call them _arcs_. These are represented by _ordered_ pairs. Such a graph is called **directed graph**. To represent a directed graph, the forms discussed above are slightly modified. 

![](./images/graph2.gif)

The example graph above is represented as follows.

_graph-term_ form:

```rust
let digraph = from_term_form(
    &vec!['r', 's', 't', 'u', 'v'],
    &vec![('s', 'r'), ('s', 'u'), ('u', 'r'), ('u', 's'), ('v', 'u')],
);
```

_adjacency-list_ form:

```rust
let digraph = from_adjacent_form(&vec![
    ('r', &vec![]), ('s', &vec!['r', 'u']), ('t', &vec![]),
    ('u', &vec!['r', 's']), ('v', &vec!['u']),
]);
```

Note that the adjacency-list does not have the information on whether it is a graph or a digraph.

_human-friendly_ form:

```
[s>r, t, u>r, s>u, u>s, v>u]
```

Finally, graphs and digraphs may have additional information attached to nodes and edges (arcs). For the nodes, this is no problem, as we can put any type into them. On the other hand, for edges we have to extend our notation. Graphs with additional information attached to edges are called **labeled graphs**.

![](./images/graph3.gif)

_graph-term_ form:

```rust
let labeled_graph = from_term_form(
    &vec!['k', 'm', 'p', 'q'],
    &vec![('p', 'm', 5), ('m', 'q', 7), ('p', 'q', 9)],
);
```

_adjacency-list_ form:

```rust
let labeled_graph = from_adjacent_form(&vec![
    ('k', &vec![]), ('m', &vec![('q', 7)]),
    ('p', &vec![('m', 5), ('q', 9)]), ('q', &vec![]),
]);
```

_human-friendly_ form:

```
[p>q/9, m>q/7, k, p>m/5]
```

The notation for labeled graphs can also be used for so-called **multi-graphs**, where more than one edge (or arc) is allowed between two given nodes.

Our Graphs use an incidence list internally. Each has a list of nodes and a list of edges. Each node also has a list of edges that connect it to other nodes. In a directed graph, nodes that are the target of arcs do not have references to those arcs in their adjacency list.

```rust
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::{Rc, Weak};

/// Graph node
#[derive(Debug, Clone)]
pub struct Node<T, U>
where
    T: Hash + Copy + Eq,
    U: Copy + Eq,
{
    value: T,
    edges: Vec<Weak<Edge<T, U>>>,
}

/// Graph Edge
#[derive(Debug, Clone)]
pub struct Edge<T, U>
where
    T: Hash + Copy + Eq,
    U: Copy + Eq,
{
    n1: Weak<RefCell<Node<T, U>>>,
    n2: Weak<RefCell<Node<T, U>>>,
    label: Option<U>,
}


/// Undirected Labeled Graph
#[derive(Debug, Clone)]
pub struct LabeledGraph<T, U>
where
    T: Hash + Copy + Eq,
    U: Copy + Eq,
{
    nodes: HashMap<T, Rc<RefCell<Node<T, U>>>>,
    edges: Vec<Rc<Edge<T, U>>>,
}

/// Undirected Graph
#[allow(dead_code)]
pub type Graph<T> = LabeledGraph<T, ()>;
```

Notice that we use `std::rc::Rc` and `std::rc::Weak` smart pointers to maintain multiple ownership. A `LabeledGraph` (or `Graph`) instance refers `Node`s and `Edge`s by `Rc<_>` pointer (strong reference); that means a graph _owns_ nodes and edges as its components. On the other hand, `Node` and `Edge` refer each other by `Weak<_>` pointer (weak reference); so that there are no reference cycles. In addition, we shall use `std::cell::RefCell` to allow immutable or mutable borrows checked at _runtime_. For more details, see documentation below:

- ["Rc<T>, the Reference Counted Smart Pointer"](https://doc.rust-lang.org/book/ch15-04-rc.html)
- ["RefCell<T> and the Interior Mutability Pattern"](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)
- ["Reference Cycles Can Leak Memory"](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html)

Full definition of the graphs are available at [here](./graph/src/lib.rs).

### [P80](./P80/src/lib.rs) (***) Conversions

Write functions to convert between the different graph representations. With these functions, all representations are equivalent; i.e. for the following problems you can always pick freely the most convenient form. The reason this problem is rated (***) is not because it's particularly difficult, but because it's a lot of work to deal with all the special cases.

For simplicity, we shall assume the type of node value is `char` and the type of edge label is `i32`.

Hint: You might need separate functions for labeled and unlabeled graphs.

[**Undirected graph representations**](./P80/src/graph_converters.rs)

Example: [examples/graph_converters.rs](./P80/examples/graph_converters.rs)
```rust
let g = unlabeled::from_string("[b-c, f-c, g-h, d, f-b, k-f, h-g]");
println!(
    "unlabeled graph (graph-term form)\n{:?}",
    unlabeled::to_term_form(&g)
);

let g = labeled::from_string("[k, m-p/5, m-q/7, p-q/9]");
println!(
    "labeled graph (adjacency-list form)\n{:?}",
    labeled::to_adjacent_form(&g)
);
```

```bash
P80 $ cargo run -q --example graph_converters
unlabeled graph (graph-term form)
(['d', 'g', 'b', 'h', 'c', 'f', 'k'], [('b', 'c'), ('g', 'h'), ('b', 'f'), ('c', 'f'), ('f', 'k')])
labeled graph (adjacency-list form)
[('p', [('m', 5), ('q', 9)]), ('q', [('m', 7), ('p', 9)]), ('m', [('p', 5), ('q', 7)]), ('k', [])]
```

[**Undirected graph representations**](./P80/src/digraph_converters.rs)

Example: [examples/digraph_converters.rs](./P80/examples/digraph_converters.rs)
```rust
let g = unlabeled::from_string("[s>r, t, u>r, s>u, u>s, v>u]");
println!(
    "unlabeled digraph (graph-term form)\n{:?}",
    unlabeled::to_term_form(&g)
);

let g = labeled::from_string("[p>q/9, m>q/7, k, p>m/5]");
println!(
    "labeled digraph (adjacency-list form)\n{:?}",
    labeled::to_adjacent_form(&g)
);
```

```bash
P80 $ cargo run -q --example digraph_converters
unlabeled digraph (graph-term form)
(['t', 'u', 'v', 's', 'r'], [('s', 'r'), ('u', 's'), ('s', 'u'), ('v', 'u'), ('u', 'r')])
labeled digraph (adjacency-list form)
[('m', [('q', 7)]), ('p', [('m', 5), ('q', 9)]), ('k', []), ('q', [])]
```

### [P81](./P81/src/lib.rs) (**) Path from one node to another one.

Write a function to find acyclic paths from one node to another in a graph. The method should return all paths. 

To handle multiple kinds of graph, we shall define `PathFinder` trait as follows. Implement `find_paths()` method for each graph (you may want to create common functions or methods to avoid duplicating the logic).

```rust
pub trait PathFinder<T> {
    fn find_paths(&self, start: char, end: char) -> Vec<Vec<T>>;
}

impl<U: Copy + Eq> PathFinder<char> for LabeledGraph<char, U> {
    fn find_paths(&self, start: char, end: char) -> Vec<Vec<char>> {
        todo!()
    }
}

impl<U: Copy + Eq> PathFinder<char> for LabeledDigraph<char, U> {
    fn find_paths(&self, start: char, end: char) -> Vec<Vec<char>> {
        todo!()
    }
}
```

Example: [examples/find_paths.rs](./P81/examples/find_paths.rs)
```rust
let g = labeled::from_string("[p>q/9, m>q/7, k, p>m/5]");
println!("Paths from p to q: {:?}", g.find_paths('p', 'q'));
println!("Paths from p to k: {:?}", g.find_paths('p', 'k'));
```

```bash
P81 $ cargo run -q --example find_paths
Paths from p to q: [['p', 'q'], ['p', 'm', 'q']]
Paths from p to k: []
```

### [P82](./P82/src/lib.rs) (*) Cycle from a given node.

Write a function named to find closed paths (cycles) starting at a given node in a graph. The method should return all cycles.

To handle multiple kinds of graph, we define `CycleFinder` trait as follows. Implement `find_cycles()` method for each graph.

```rust
pub trait CycleFinder<T> {
    fn find_cycles(&self, node: char) -> Vec<Vec<char>>;
}

impl<U: Copy + Eq> CycleFinder<char> for LabeledGraph<char, U> {
    fn find_cycles(&self, node: char) -> Vec<Vec<char>> {
        todo!()
    }
}

impl<U: Copy + Eq> CycleFinder<char> for LabeledDigraph<char, U> {
    fn find_cycles(&self, node: char) -> Vec<Vec<char>> {
        todo!()
    }
}
```

Example: [examples/find_cycles.rs](./P82/examples/find_cycles.rs)
```rust
let g = unlabeled::from_string("[b-c, f-c, g-h, d, f-b, k-f, h-g]");
println!("Cycles starting at f: {:?}", g.find_cycles('f'));
println!("Cycles starting at g: {:?}", g.find_cycles('g'));
```

```rust
P82 $ cargo run -q --example find_cycles
Cycles starting at f: [['f', 'b', 'c', 'f'], ['f', 'c', 'b', 'f']]
Cycles starting at g: []
```

### [P83](./P83/src/lib.rs) (**) Construct all spanning trees.

Write a function `spanning_trees()` to construct all [spanning trees](https://en.wikipedia.org/wiki/Spanning_tree) of a given graph. With this method, find out how many spanning trees there are for the graph below. The data of this example graph can be found below. When you have a correct solution for the spanningTrees method, use it to define two other useful methods: `is_tree()` and `is_connected()`. Both are five-minute tasks! 

![](./images/p83.gif)

Example: [examples/spanning_trees.rs](./P83/examples/spanning_trees.rs)
```rust
let g = unlabeled::from_term_form(
    &vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'],
    &vec![
        ('a', 'b'), ('a', 'd'), ('b', 'c'), ('b', 'e'), ('c', 'e'),
        ('d', 'e'), ('d', 'f'), ('d', 'g'), ('e', 'h'), ('f', 'g'), 
        ('g', 'h'),
    ],
);
let trees = spanning_trees(&g);
for tree in trees {
    println!("{:?}", unlabeled::to_term_form(&tree));
}
```

```bash
P83 $ cargo run -q --example spanning_trees
(['h', 'b', 'a', 'd', 'e', 'c', 'f', 'g'], [('g', 'h'), ('f', 'g'), ('d', 'e'), ('c', 'e'), ('d', 'f'), ('b', 'c'), ('a', 'b')])
(['h', 'f', 'a', 'c', 'd', 'g', 'e', 'b'], [('a', 'd'), ('f', 'g'), ('g', 'h'), ('c', 'e'), ('d', 'e'), ('d', 'f'), ('b', 'c')])
(['a', 'd', 'f', 'b', 'g', 'h', 'c', 'e'], [('c', 'e'), ('d', 'f'), ('f', 'g'), ('g', 'h'), ('a', 'b'), ('d', 'e'), ('b', 'e')])
(['g', 'a', 'd', 'f', 'b', 'e', 'h', 'c'], [('c', 'e'), ('g', 'h'), ('d', 'f'), ('d', 'e'), ('a', 'd'), ('f', 'g'), ('b', 'e')])
......
```

### [P84](./P84/src/lib.rs) (**) Construct the minimal spanning tree.

Write a function `minimal_spanning_tree()` to construct the [minimal spanning tree](https://en.wikipedia.org/wiki/Minimum_spanning_tree) of given labeled graph. 

Hint: Use Prim's Algorithm. A small modification of the solution of P83 does the trick. The data of the example graph can be found below.

![](./images/p84.gif)

Example: [examples/minimal_spanning_trees.rs](./P84/examples/minimal_spanning_trees.rs)
```rust
    let g = labeled::from_term_form(
        &vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'],
        &vec![
            ('a', 'b', 5), ('a', 'd', 3), ('b', 'c', 2), ('b', 'e', 4), ('c', 'e', 6),
            ('d', 'e', 7), ('d', 'f', 4), ('d', 'g', 3), ('e', 'h', 5), ('f', 'g', 4),
            ('g', 'h', 1),
        ],
    );
    let trees = minimal_spanning_trees(&g);
    for tree in trees {
        println!("{:?} (weight={})", labeled::to_term_form(&tree), label_sum(&tree));
    }
```

```bash
P84 $ cargo run -q --example minimal_spanning_trees
(['c', 'f', 'h', 'e', 'd', 'g', 'a', 'b'], [('b', 'c', 2), ('g', 'h', 1), ('b', 'e', 4), ('d', 'g', 3), ('a', 'd', 3), ('e', 'h', 5), ('f', 'g', 4)]) (weight=22)
(['f', 'd', 'g', 'c', 'e', 'a', 'b', 'h'], [('b', 'e', 4), ('d', 'f', 4), ('a', 'b', 5), ('a', 'd', 3), ('g', 'h', 1), ('d', 'g', 3), ('b', 'c', 2)]) (weight=22)
```

### [P85](./P85/src/lib.rs) (**) Graph isomorphism.

Two graphs G1(N1,E1) and G2(N2,E2) are isomorphic if there is a bijection f: N1 → N2 such that for any nodes X,Y of N1, X and Y are adjacent if and only if f(X) and f(Y) are adjacent.

Write a function that determines whether two graphs are isomorphic.

Example: [examples/is_isomorphic_to.rs](./P85/examples/is_isomorphic_to.rs)
```rs
let g1 = unlabeled::from_string("[a-b b-c]");
let g2 = unlabeled::from_string("[5-7 9-7]");
println!(
    "{:?} is isomorphic to {:?}: {}", 
    unlabeled::to_term_form(&g1), unlabeled::to_term_form(&g2), is_isomorphic_to(&g1, &g2)
);

let g1 = unlabeled::from_string("[a-b b-c c-d]");
let g2 = unlabeled::from_string("[5-7 9-7 7-3]");
println!(
    "{:?} is isomorphic to {:?}: {}",
    unlabeled::to_term_form(&g1), unlabeled::to_term_form(&g2), is_isomorphic_to(&g1, &g2)
);
```

```bash
P85 $ cargo run -q --example is_isomorphic_to
(['a', 'c', 'b'], [('a', 'b'), ('b', 'c')]) is isomorphic to (['5', '9', '7'], [('5', '7'), ('7', '9')]): true
(['b', 'd', 'a', 'c'], [('b', 'c'), ('a', 'b'), ('c', 'd')]) is isomorphic to (['7', '9', '3', '5'], [('5', '7'), ('7', '9'), ('3', '7')]): false
```

### [P86](./P86/src/lib.rs) (**) Node degree and graph coloration.

a) Write a method `degree()` for Node that determines the degree of a given node.

Example: [examples/degree.rs](./P86/examples/degree.rs)
```rs
let g = unlabeled::from_string("[a-b, b-c, a-c, a-d]");
println!("degree of node 'a' = {}", g.get_node(&'a').unwrap().degree());
```

```bash
P86 $ cargo run -q --example degree
degree of node 'a' = 3
```

b) Write a method for LabeledGraph that lists all nodes of a graph sorted according to ascending or decreasing degree; if there are nodes that have a same degree, sort them by their values.

Example: [examples/nodes_by_degree.rs](./P86/examples/nodes_by_degree.rs)
```rs
let g = unlabeled::from_string("[a-b, b-c, a-c, a-d]");
println!("{:?}", g.get_nodes_by_degree(true));
```

```bash
P86 $ cargo run -q --example nodes_by_degree
[Node { value: 'a', edges: [(Weak), (Weak), (Weak)] }, Node { value: 'b', edges: [(Weak), (Weak)] }, Node { value: 'c', edges: [(Weak), (Weak)] }, Node { value: 'd', edges: [(Weak)] }]
```

c) Use Welsh-Powell's algorithm to paint the nodes of a graph in such a way that adjacent nodes have different colors. Make a function `color_nodes()` that returns a list of tuples, each of which contains a node value and an integer representing its color.

Example: [examples/color_nodes.rs](./P86/examples/color_nodes.rs)
```rs
let g = unlabeled::from_string("[a-b, b-c, a-c, a-d]");
println!("{:?}", color_nodes(&g));
```

```bash
P86 $ cargo run -q --example color_nodes
[('a', 1), ('b', 2), ('d', 2), ('c', 3)]
```

### [P87](./P87/src/lib.rs) (**) Depth-first order graph traversal.

Write a function that generates a depth-first order graph traversal sequence. The starting point should be specified, and the output should be a list of nodes that are reachable from this starting point (in depth-first order). 

Example: [examples/depth_first.rs](./P87/examples/depth_first.rs)
```rust
let g = unlabeled::from_string("[a-b, b-c, e, a-c, a-d]");
println!("{:?}", nodes_by_depth_from(&g, 'd'));
```

```bash
P87 $ cargo run -q --example depth_first
['c', 'b', 'a', 'd']
```

### [P88](./P88/src/lib.rs) (**) Connected components.

Write a function `split_graph()` that splits a graph into its connected components.

Example: [examples/split_graph.rs](./P88/examples/split_graph.rs)
```rust
let splitted = split_graph(&unlabeled::from_string("[a-b, b-c, d, e-f, f-g, g-e, h]"));
for g in splitted {
    println!("{:?}", unlabeled::to_term_form(&g));
}
```

```bash
P88 $ cargo run -q --example split_graph
(['c', 'b', 'a'], [('b', 'c'), ('a', 'b')])
(['h'], [])
(['g', 'e', 'f'], [('e', 'f'), ('f', 'g'), ('e', 'g')])
(['d'], [])
```

### [P89](./P89/src/lib.rs) (**) Bipartite graphs.

Write a function `is_bipartite()` that determines whether a given graph is [bipartite](https://en.wikipedia.org/wiki/Bipartite_graph).

Hint: a graph is bipartite if and only if it is two-colorable: https://cp-algorithms.com/graph/bipartite-check.html

Example: [examples/is_bipartite.rs](./P89/examples/is_bipartite.rs)
```rust
let g = unlabeled::from_string("[a-b, b-c, d, e-f, f-g, g-e, h]");
println!(
    "{:?} is bipartite: {}",
    unlabeled::to_term_form(&g),
    is_bipartite(&g)
);
let g = unlabeled::from_string("[a-b, b-c, d, e-f, g-e, h]");
println!(
    "{:?} is bipartite: {}",
    unlabeled::to_term_form(&g),
    is_bipartite(&g)
);
```

```bash
P89 $ cargo run -q --example is_bipartite
(['b', 'a', 'h', 'e', 'g', 'c', 'd', 'f'], [('b', 'c'), ('e', 'f'), ('a', 'b'), ('e', 'g'), ('f', 'g')]) is bipartite: false
(['h', 'a', 'd', 'e', 'f', 'g', 'b', 'c'], [('e', 'f'), ('a', 'b'), ('e', 'g'), ('b', 'c')]) is bipartite: true
```
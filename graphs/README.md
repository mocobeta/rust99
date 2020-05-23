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

Notice that we use `std::rc::Rc` and `std::rc::Weak` smart pointers to maintain multiple ownership. A `LabeledGraph` (or `Graph`) instance refers `Node`s and `Edge`s by `Rc<_>` pointer (strong reference); that means a graph _owns_ nodes and edges as its components. On the other hand, `Node` and `Edge` refer each other by `Weak<_>` pointer (weak reference); so that there are no reference cycles. In addition, we shall use `std::cell::RefCell` to allow immutable or mutable borrows checked at _runtime_. For more details, see references below:

- ["Rc<T>, the Reference Counted Smart Pointer"](https://doc.rust-lang.org/book/ch15-04-rc.html)
- ["RefCell<T> and the Interior Mutability Pattern"](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)
- ["Reference Cycles Can Leak Memory"](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html)

Full definition of the graphs are available at [here](./graph/src/lib.rs).

### [P80](./P80/src/lib.rs) (***) Conversions

Write functions to convert between the different graph representations. With these functions, all representations are equivalent; i.e. for the following problems you can always pick freely the most convenient form. The reason this problem is rated (***) is not because it's particularly difficult, but because it's a lot of work to deal with all the special cases.

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

### P81 (**) Path from one node to another one.

### P82 (*) Cycle from a given node.

### P83 (**) Construct all spanning trees.

### P84 (**) Construct the minimal spanning tree.

### P85 (**) Graph isomorphism.

### P86 (**) Node degree and graph coloration.

### P87 (**) Depth-first order graph traversal.

### P88 (**) Connected components.

### P89 (**) Bipartite graphs.
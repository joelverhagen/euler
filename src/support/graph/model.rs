pub type EdgeIndex = usize;
pub type NodeIndex = usize;

pub struct Node<N> {
    first_outgoing_edge: Option<EdgeIndex>,
    value: N,
}

impl<N> Node<N> {
    pub fn value(&self) -> &N {
        &self.value
    }
}

pub struct Edge<E> {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>,
    value: E,
}

impl<E> Edge<E> {
    pub fn value(&self) -> &E {
        &self.value
    }
}

pub struct Graph<N, E> {
    nodes: Vec<Node<N>>,
    edges: Vec<Edge<E>>,
}

impl<N, E> Graph<N, E> {
    pub fn new() -> Graph<N, E> {
        Graph { nodes: Vec::new(), edges: Vec::new() }
    }

    pub fn add_node(&mut self, value: N) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(Node { first_outgoing_edge: None, value: value });
        index
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex, value: E) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(Edge {
            target: target,
            next_outgoing_edge: node_data.first_outgoing_edge,
            value: value
        });
        node_data.first_outgoing_edge = Some(edge_index);
    }

    pub fn nodes(&self) -> &Vec<Node<N>> {
        &self.nodes
    }

    pub fn node(&self, index: NodeIndex) -> &Node<N> {
        &self.nodes[index]
    }

    pub fn edge(&self, index: EdgeIndex) -> &Edge<E> {
        &self.edges[index]
    }

    pub fn successors(&self, source: NodeIndex) -> Successors<N, E> {
        let first_outgoing_edge = self.nodes[source].first_outgoing_edge;
        Successors { graph: self, current_edge_index: first_outgoing_edge }
    }
}

struct SuccessorNode {
    edge: EdgeIndex,
    node: NodeIndex,
}

impl SuccessorNode {
    pub fn edge(&self) -> EdgeIndex {
        self.edge
    }

    pub fn node(&self) -> NodeIndex {
        self.node
    }
}

struct Successors<'graph, N: 'graph, E: 'graph> {
    graph: &'graph Graph<N, E>,
    current_edge_index: Option<EdgeIndex>,
}

impl<'graph, N, E> Iterator for Successors<'graph, N, E> {
    type Item = SuccessorNode;

    fn next(&mut self) -> Option<SuccessorNode> {
        match self.current_edge_index {
            None => None,
            Some(edge_index) => {
                let edge = &self.graph.edges[edge_index];
                self.current_edge_index = edge.next_outgoing_edge;
                Some(SuccessorNode { edge: edge_index, node: edge.target })
            }
        }
    }
}

use support::triangles::Triangles;

use std::collections::VecDeque;
use std::collections::HashSet;

type EdgeIndex = usize;
type NodeIndex = usize;

struct Node {
    first_outgoing_edge: Option<EdgeIndex>,
}

struct Edge {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>,
}

struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph {
    fn new() -> Graph {
        Graph { nodes: Vec::new(), edges: Vec::new() }
    }

    fn add_node(&mut self) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(Node { first_outgoing_edge: None });
        index
    }

    fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(Edge {
            target: target,
            next_outgoing_edge: node_data.first_outgoing_edge
        });
        node_data.first_outgoing_edge = Some(edge_index);
    }

    fn successors(&self, source: NodeIndex) -> Successors {
        let first_outgoing_edge = self.nodes[source].first_outgoing_edge;
        Successors { graph: self, current_edge_index: first_outgoing_edge }
    }
}

struct Successors<'graph> {
    graph: &'graph Graph,
    current_edge_index: Option<EdgeIndex>,
}

impl<'graph> Iterator for Successors<'graph> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<NodeIndex> {
        match self.current_edge_index {
            None => None,
            Some(edge_num) => {
                let edge = &self.graph.edges[edge_num];
                self.current_edge_index = edge.next_outgoing_edge;
                Some(edge.target)
            }
        }
    }
}

fn breadth_first_search(graph: &Graph, start: NodeIndex) {
    let mut nodes: VecDeque<NodeIndex> = VecDeque::new();
    let mut visited: HashSet<NodeIndex> = HashSet::new();
    nodes.push_back(start);

    while nodes.len() > 0 {
        let node = nodes.pop_front().unwrap();
        if visited.contains(&node) {
            continue;
        }
        
        visited.insert(node);
        for successor in graph.successors(node) {
            if visited.contains(&successor) {
                continue;
            }

            nodes.push_back(successor);
        }
    }
}

fn parse_triangle_graph(node_count: usize) -> Graph {
    let mut graph = Graph::new();

    let mut layer = 0;
    for triangle in Triangles::iter().skip(1) {
        if triangle as usize > node_count {
            break;
        }

        // initialize the nodes
        let last_triangle = triangle - (layer + 1);
        for i in last_triangle..triangle {
            graph.add_node();
        }
        
        // initialize edges
        if layer > 0 {
            let last_layer_start = last_triangle - layer;
            let last_layer_up_to = last_triangle;
            for i in last_layer_start..last_layer_up_to {
                graph.add_edge(i as NodeIndex, (i + layer) as NodeIndex);
                graph.add_edge(i as NodeIndex, (i + layer + 1) as NodeIndex);
            }
        }
        
        layer += 1;
    }    

    graph
}

// 75
// 95 64
// 17 47 82
// 18 35 87 10
// 20 04 82 47 65
// 19 01 23 75 03 34
// 88 02 77 73 07 63 67
// 99 65 04 28 06 16 70 92
// 41 41 26 56 83 40 80 70 33
// 41 48 72 33 47 32 37 16 94 29
// 53 71 44 65 25 43 91 52 97 51 14
// 70 11 33 28 77 73 17 78 39 68 17 57
// 91 71 52 38 17 14 91 43 58 50 27 29 48
// 63 66 04 68 89 53 67 30 73 16 69 87 40 31
// 04 62 98 27 23 09 70 98 73 93 38 53 60 04 23

#[allow(dead_code)]
pub fn get_answer() -> i32 {
    /*
    let mut graph = Graph::new();
    let n0 = graph.add_node();
    let n1 = graph.add_node();
    let n2 = graph.add_node();
    let n3 = graph.add_node();
    let n4 = graph.add_node();
    let n5 = graph.add_node();

    graph.add_edge(n0, n1);
    graph.add_edge(n0, n2);
    graph.add_edge(n1, n3);
    graph.add_edge(n1, n4);
    graph.add_edge(n2, n4);
    graph.add_edge(n2, n5);

    let mut n = n0;

    breadth_first_search(&graph, n0);
    */
    /*
    let input = r#"75
                   95 64
                   17 47 82
                   18 35 87 10
                   20 04 82 47 65
                   19 01 23 75 03 34
                   88 02 77 73 07 63 67
                   99 65 04 28 06 16 70 92
                   41 41 26 56 83 40 80 70 33
                   41 48 72 33 47 32 37 16 94 29
                   53 71 44 65 25 43 91 52 97 51 14
                   70 11 33 28 77 73 17 78 39 68 17 57
                   91 71 52 38 17 14 91 43 58 50 27 29 48
                   63 66 04 68 89 53 67 30 73 16 69 87 40 31
                   04 62 98 27 23 09 70 98 73 93 38 53 60 04 23"#;
    */
    let input = r#"75
                   95 64
                   17 47 82
                   18 35 87 10
                   20 04 82 47 65
                   19 01 23 75 03 34"#;

    let numbers: Vec<i32> = input
        .lines()
        .flat_map(|l| l.split(' '))
        .map(|p| p.trim())
        .filter(|&p| p.len() > 0)
        .map(|p| p.parse::<i32>().unwrap())        
        .collect();

    let graph = parse_triangle_graph(numbers.len());

    0
}

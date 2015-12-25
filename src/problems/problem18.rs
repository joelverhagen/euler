use support::triangles::Triangles;
use support::graph::{Graph, NodeIndex};

use std::collections::VecDeque;
use std::collections::HashSet;

fn breadth_first_search<N, E>(graph: &Graph<N, E>, start: NodeIndex) -> Vec<&N> {
    let mut nodes: VecDeque<NodeIndex> = VecDeque::new();
    let mut visited: HashSet<NodeIndex> = HashSet::new();
    let mut values: Vec<&N> = Vec::new();
    nodes.push_back(start);

    while nodes.len() > 0 {
        let node_index = nodes.pop_front().unwrap();
        if visited.contains(&node_index) {
            continue;
        }

        values.push(graph.nodes()[node_index].value());
        
        visited.insert(node_index);
        for successor in graph.successors(node_index) {
            if visited.contains(&successor) {
                continue;
            }

            nodes.push_back(successor);
        }
    }

    values
}

fn parse_triangle_graph<E>(values: Vec<E>) -> Graph<E, E> where E : Copy {
    let mut graph = Graph::new();
    let value_count = values.len();
    let mut remaining_values = values.to_vec();
    remaining_values.reverse();

    let mut layer = 0;
    for triangle in Triangles::iter().skip(1) {
        if triangle as usize > value_count {
            break;
        }

        // initialize the nodes
        let last_triangle = triangle - (layer + 1);
        for _ in last_triangle..triangle {
            let value = remaining_values.pop().unwrap();
            graph.add_node(value);
        }
        
        // initialize edges
        if layer > 0 {
            let last_layer_start = last_triangle - layer;
            let last_layer_up_to = last_triangle;
            for i in last_layer_start..last_layer_up_to {
                let index_left = (i + layer) as NodeIndex;
                let value_left = *graph.node(index_left).value();
                graph.add_edge(i as NodeIndex, index_left, value_left);

                let index_right = (i + layer + 1) as NodeIndex;
                let value_right = *graph.node(index_right).value();
                graph.add_edge(i as NodeIndex, index_right, value_right);
            }
        }
        
        layer += 1;
    }    

    graph
}

#[allow(dead_code)]
pub fn get_answer() -> i32 {
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
    let input = r#"

                     3
                    7 4
                   2 4 6
                  8 5 9 3

"#;

    let numbers: Vec<i32> = input
        .lines()
        .flat_map(|l| l.split(' '))
        .map(|p| p.trim())
        .filter(|&p| p.len() > 0)
        .map(|p| p.parse::<i32>().unwrap())        
        .collect();

    let graph = parse_triangle_graph(numbers);
    for value in breadth_first_search(&graph, 0) {
        println!("{}", value);
    }

    0
}

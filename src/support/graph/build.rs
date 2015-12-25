use support::triangles::Triangles;
use support::graph::model::{Graph, NodeIndex};

pub fn build_triangle_graph<E>(values: Vec<E>) -> Graph<E, E> where E : Copy {
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

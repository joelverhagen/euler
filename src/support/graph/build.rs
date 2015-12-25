use support::triangles::Triangles;
use support::graph::model::{Graph, NodeIndex};

pub struct TriangleGraph<E> {
    extra_values: Vec<E>,
    layers: usize,
    graph: Graph<E, E>,
}

impl<E> TriangleGraph<E> {
    pub fn new(values: &Vec<E>) -> TriangleGraph<E> where E : Copy {
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

        TriangleGraph { extra_values: remaining_values, layers: layer as usize, graph: graph }
    }

    pub fn graph(&self) -> &Graph<E, E> {
        &self.graph
    }

    #[allow(dead_code)]
    pub fn extra_values(&self) -> &Vec<E> {
        &self.extra_values
    }

    pub fn layers(&self) -> usize {
        self.layers
    }
}

use support::graph::build::TriangleGraph;
use support::graph::search::Dijkstra;
use support::graph::model::NodeIndex;

#[allow(dead_code)]
pub fn get_answer() -> i32 {
    let input_a = r#"75
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
    /*
    let input_b = r#"3
                     7 4
                     2 4 6
                     8 5 9 3"#;
    */

    // parse the input
    let numbers: Vec<i32> = input_a
        .lines()
        .flat_map(|l| l.split(' '))
        .map(|p| p.trim())
        .filter(|&p| p.len() > 0)
        .map(|p| p.parse::<i32>().unwrap())        
        .collect();

    // make the smallest numbers the largest because Dijkstra's finds a minimum path
    let max_number = numbers.iter().max().unwrap();
    let reversed_numbers = numbers.iter().map(|n| max_number - n).collect();

    // build the triangle graph
    let triangle_graph = TriangleGraph::new(&reversed_numbers);

    // run Dijkstra's
    let dijkstra = Dijkstra::new(triangle_graph.graph(), 0);

    // find all of the nodes in that last layer
    let mut nodes_with_paths: Vec<NodeIndex> = dijkstra
        .dist()
        .iter()
        .filter(|&(_, v)| v.is_some())
        .map(|(k, _)| *k)
        .collect();
    nodes_with_paths.sort();
    nodes_with_paths.reverse();
    let last_layer_nodes: Vec<NodeIndex> = nodes_with_paths
        .iter()
        .take(triangle_graph.layers())
        .cloned()
        .collect();

    // find the max path
    let g = triangle_graph.graph();
    last_layer_nodes
        .iter()
        .map(|n| dijkstra.get_path(*n).unwrap())
        .map(|p| p.iter().fold(0, |s, n| s + (max_number - g.node(*n).value())))
        .max()
        .unwrap()
}

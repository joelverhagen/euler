use support::graph::model::{Graph, NodeIndex};
use std::collections::{VecDeque, HashSet, HashMap};

#[allow(dead_code)]
pub fn breadth_first_search<N, E>(graph: &Graph<N, E>, start: NodeIndex) -> Vec<&N> {
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
            if visited.contains(&successor.node()) {
                continue;
            }

            nodes.push_back(successor.node());
        }
    }

    values
}

pub struct Dijkstra {
    q: HashSet<NodeIndex>,
    dist: HashMap<NodeIndex, Option<i32>>,
    prev: HashMap<NodeIndex, Option<NodeIndex>>,
}

struct NodeIndexAndDistance {
    node: NodeIndex,
    dist: i32,
}

impl Dijkstra {
    fn initialize<N>(graph: &Graph<N, i32>, start: NodeIndex) -> Dijkstra {
        let mut q = HashSet::new();
        let mut dist = HashMap::new();
        let mut prev = HashMap::new();

        let node_count = graph.nodes().len();
        for v in 0..node_count {
            dist.insert(v, None);
            prev.insert(v, None);
            q.insert(v);
        }

        *dist.get_mut(&start).unwrap() = Some(0);

        Dijkstra { q: q, dist: dist, prev: prev }
    }

    fn get_min_node(&self) -> NodeIndexAndDistance {
        let mut u = *self.q.iter().nth(0).unwrap();
        let mut u_dist_opt = self.dist.get(&u).unwrap();
        for v in &self.q {
            let v_dist_opt = self.dist.get(&v).unwrap();
            if v_dist_opt.is_none() {
                continue;
            }

            if u_dist_opt.is_none() || v_dist_opt.unwrap() < u_dist_opt.unwrap() {
                u = *v;
                u_dist_opt = v_dist_opt;
            }
        }

        NodeIndexAndDistance { node: u, dist: u_dist_opt.unwrap() }
    }

    #[allow(dead_code)]
    pub fn new<N>(graph: &Graph<N, i32>, start: NodeIndex) -> Dijkstra {
        let mut d = Dijkstra::initialize(graph, start);

        while d.q.len() > 0 {

            let NodeIndexAndDistance { node: u, dist: u_dist } = d.get_min_node();
            
            d.q.remove(&u);

            for s in graph.successors(u) {
                let v = s.node();
                let alt = u_dist + graph.edge(s.edge()).value();
                let v_dist_opt: Option<i32>;
                { v_dist_opt = *d.dist.get(&v).unwrap(); }

                if v_dist_opt.is_none() || alt < v_dist_opt.unwrap() {
                    *d.dist.get_mut(&v).unwrap() = Some(alt);
                    *d.prev.get_mut(&v).unwrap() = Some(u);
                }
            }
        }

        d
    }

    #[allow(dead_code)]
    pub fn dist(&self) -> &HashMap<NodeIndex, Option<i32>> {
        &self.dist
    }

    #[allow(dead_code)]
    pub fn prev(&self) -> &HashMap<NodeIndex, Option<NodeIndex>> {
        &self.prev
    }

    pub fn get_path(&self, end: NodeIndex) -> Option<Vec<NodeIndex>> {
        if self.prev[&end].is_none() {
            None
        } else {
            let mut path = Vec::new();
            let mut curr = Some(end);
            while curr.is_some() {
                path.push(curr.unwrap());
                curr = self.prev[&curr.unwrap()];
            }

            path.reverse();

            Some(path)
        }
    }
}

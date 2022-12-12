use std::collections::VecDeque;
use std::collections::HashSet;

pub struct Graph {
    nodes: Vec<NodeData>,
    edges: Vec<EdgeData>,
}

pub type NodeIndex = usize;

#[derive(Debug)]
pub struct NodeData {
    first_outgoing_edge: Option<EdgeIndex>,
    shortest_path: Option<u32>
}

pub type EdgeIndex = usize;

pub struct EdgeData {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>
}

impl Graph {
    pub fn new() -> Graph {
        let nodes = Vec::new();
        let edges = Vec::new();

        Graph { nodes, edges } 
    }

    pub fn add_node(&mut self) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(NodeData { first_outgoing_edge: None, 
                                   shortest_path: None});
        index
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        let edge_index = self.edges.len();
        let node_data = &mut self.nodes[source];
        self.edges.push(EdgeData {
            target: target,
            next_outgoing_edge: node_data.first_outgoing_edge
        });
        node_data.first_outgoing_edge = Some(edge_index);
    }

    pub fn successors(&self, source: NodeIndex) -> Successors {
        let first_outgoing_edge = self.nodes[source].first_outgoing_edge;
        Successors { graph: self, current_edge_index: first_outgoing_edge }
    }

    pub fn shortest_path(&mut self, start: NodeIndex, end: NodeIndex) -> u32 {
        // Implements Dijkstra's shortest path algorithm. 
        // Currently assumes weightings are all 1.
        let mut queue: VecDeque<(NodeIndex, u32)> = VecDeque::from([(start, 0)]);

        loop {
            let (current, current_distance) = match queue.pop_front() {
                None => {break;},
                Some(x) => x,
            };

            let node_data = &mut self.nodes[current];
            let update_current = match node_data.shortest_path {
                None => true, 
                Some(x) => x > current_distance
            };
            if update_current {
                node_data.shortest_path = Some(current_distance);
                for successor_node in self.successors(current) {
                    queue.push_back((successor_node, current_distance + 1)); // ToDo: include weights
                }
            }
        }
        
        let target_node = &mut self.nodes[end];
        return target_node.shortest_path.unwrap();
    }

    pub fn bfs(&mut self, start: NodeIndex, end: NodeIndex) -> Option<u32> {
        // Implements Breadth First Search algorithm. 
        let mut queue: VecDeque<(NodeIndex, u32)> = VecDeque::from([(start, 0)]);
        let mut visited = HashSet::from([start]);

        loop {
            let (current, current_distance) = match queue.pop_front() {
                None => {return None;},
                Some(x) => x,
            };

            if current == end {
                return Some(current_distance)
            }

            for successor_node in self.successors(current) {
                if ! visited.contains(&successor_node) {
                    visited.insert(successor_node);
                    queue.push_back((successor_node, current_distance + 1));
                }
            }
        }
    }
}

pub struct Successors<'graph> {
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
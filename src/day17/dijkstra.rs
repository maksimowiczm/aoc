use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::Add;

// this is useless, but I tried to use this :)

#[derive(Debug)]
pub struct Graph<VID, V, E> {
    vertices: HashMap<VID, V>,
    adjacency: HashMap<VID, Vec<(VID, E)>>,
}

impl<VID, V, E> Graph<VID, V, E>
where
    VID: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            adjacency: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, id: VID, vertex: V) {
        self.vertices.insert(id, vertex);
    }

    pub fn add_edge(&mut self, from: VID, to: VID, edge: E) {
        self.adjacency
            .entry(from)
            .or_insert_with(Vec::new)
            .push((to, edge));
    }

    pub fn get_neighbors(&self, id: &VID) -> Option<&Vec<(VID, E)>> {
        self.adjacency.get(id)
    }
}

impl<VID, V, E> Graph<VID, V, E>
where
    E: Add<Output = E> + Default + Copy + PartialOrd,
    VID: Eq + Hash + Copy,
{
    pub fn dijkstra(&self, start: VID) -> HashMap<VID, E> {
        let mut distances = HashMap::new();
        distances.insert(start, E::default());
        let mut to_visit = vec![start];
        let mut visited = HashSet::new();

        loop {
            let current = match to_visit.pop() {
                Some(current) => current,
                None => break,
            };

            let neighbors = match self.adjacency.get(&current) {
                Some(neighbors) => neighbors,
                None => continue,
            };

            for (neighbor, edge) in neighbors {
                let distance = distances[&current] + *edge;

                distances
                    .entry(*neighbor)
                    .and_modify(|d| {
                        if distance < *d {
                            *d = distance;
                        }
                    })
                    .or_insert(distance);
            }

            visited.insert(current);
            to_visit.pop();

            for neighbor in neighbors.iter().map(|(neighbor, _)| neighbor) {
                if !visited.contains(neighbor) {
                    to_visit.push(*neighbor);
                }
            }
        }

        distances
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dijkstra() {
        let mut graph = Graph::new();
        graph.add_vertex('A', ());
        graph.add_vertex('B', ());
        graph.add_vertex('C', ());
        graph.add_vertex('D', ());
        graph.add_vertex('E', ());

        graph.add_edge('A', 'B', 4);
        graph.add_edge('A', 'C', 2);

        graph.add_edge('B', 'C', 3);
        graph.add_edge('B', 'D', 2);
        graph.add_edge('B', 'E', 3);

        graph.add_edge('C', 'B', 1);
        graph.add_edge('C', 'D', 4);
        graph.add_edge('C', 'E', 5);

        graph.add_edge('E', 'D', 1);

        let distances = graph.dijkstra('A');

        assert_eq!(distances.get(&'A'), Some(&0));
        assert_eq!(distances.get(&'B'), Some(&3));
        assert_eq!(distances.get(&'C'), Some(&2));
        assert_eq!(distances.get(&'D'), Some(&5));
        assert_eq!(distances.get(&'E'), Some(&6));
    }
}

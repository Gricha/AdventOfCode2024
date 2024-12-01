use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;

pub struct Neighbor<N> {
    pub node: N,
    pub step_cost: u32,
}

pub type Neighbors<N> = Box<dyn Fn(&N) -> Vec<Neighbor<N>>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct NodeWithCost<N>
where
    N: PartialEq + Eq + Hash + Clone,
{
    node: N,
    cost: u32,
}

impl<N> PartialOrd for NodeWithCost<N>
where
    N: PartialEq + Eq + Hash + Clone,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cost.cmp(&other.cost).reverse())
    }
}

impl<N> Ord for NodeWithCost<N>
where
    N: PartialEq + Eq + Hash + Clone,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

pub fn dijkstra<N>(
    source: N,
    get_neighbors: Neighbors<N>,
    reached_end: Box<dyn Fn(&N) -> bool>,
) -> u32
where
    N: PartialEq + Eq + Hash + Clone,
{
    let mut visited = HashSet::<N>::new();
    let mut queue = BinaryHeap::<NodeWithCost<N>>::new();
    let mut dist = HashMap::<N, u32>::new();
    dist.insert(source.clone(), 0u32);

    queue.push(NodeWithCost {
        node: source,
        cost: 0,
    });

    while let Some(NodeWithCost { node, cost }) = queue.pop() {
        if !visited.insert(node.clone()) {
            continue;
        }

        if reached_end(&node) {
            return cost;
        }

        let neighbors = get_neighbors(&node);

        for neighbor in neighbors {
            let new_cost = cost + neighbor.step_cost;

            if !dist.contains_key(&neighbor.node) || new_cost < *dist.get(&neighbor.node).unwrap() {
                dist.insert(neighbor.node.clone(), new_cost);
                queue.push(NodeWithCost {
                    node: neighbor.node,
                    cost: new_cost,
                });
            }
        }
    }

    unreachable!()
}

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::Display;
use std::fs;
use std::str::FromStr;

#[derive(Copy, Eq, Clone, PartialEq)]
struct Edge {
    position: (usize, usize),
    cost: usize,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct CaveMap<const N: usize> {
    cave: [[u8; N]; N],
}

impl<const N: usize> From<String> for CaveMap<N> {
    fn from(s: String) -> Self {
        let mut cave: [[u8; N]; N] = [[0; N]; N];

        for (row_id, row) in s.lines().enumerate() {
            for (col_id, col) in row.chars().enumerate() {
                cave[row_id][col_id] = col.to_digit(10).unwrap() as u8;
            }
        }

        CaveMap { cave }
    }
}

impl<const N: usize> Display for CaveMap<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.cave {
            let line: String = row.iter().map(|v| v.to_string()).collect();
            writeln!(f, "{}", line)?;
        }

        Ok(())
    }
}

impl<const N: usize> FromStr for CaveMap<N> {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cave: [[u8; N]; N] = [[0; N]; N];

        for (row_id, row) in s.lines().enumerate() {
            for (col_id, col) in row.chars().enumerate() {
                cave[row_id][col_id] = col.to_digit(10).unwrap() as u8;
            }
        }

        Ok(CaveMap { cave })
    }
}

impl<const N: usize> CaveMap<N> {
    fn shortest_path_to_bottom(&self) -> usize {
        // Dijkstra algorithm
        let mut distance: HashMap<(usize, usize), usize> = HashMap::new();
        let mut active_nodes: BinaryHeap<Edge> = Default::default();
        distance.insert((0, 0), 0);
        let initial_path = Edge {
            cost: 0,
            position: (0, 0),
        };
        active_nodes.push(initial_path);

        while let Some(Edge { position, cost }) = active_nodes.pop() {
            // found bottom right corner
            if position == (N - 1, N - 1) {
                return cost;
            }

            // check if we already found a better path
            let curr_dist = distance.entry(position).or_insert(usize::MAX);
            if cost > *curr_dist {
                continue;
            }

            // check the neighbors
            for (neighbor_pos, neighbor_cost) in self.neighbors_of(&position) {
                let next = Edge {
                    cost: cost + neighbor_cost as usize,
                    position: neighbor_pos,
                };

                let neighbor_dist = distance.entry(neighbor_pos).or_insert(usize::MAX);
                if next.cost < *neighbor_dist {
                    active_nodes.push(next);
                    *neighbor_dist = next.cost;
                }
            }
        }
        unreachable!()
    }

    fn neighbors_of(&self, cell: &(usize, usize)) -> Vec<((usize, usize), u8)> {
        let mut neighbors: Vec<((usize, usize), u8)> = vec![];
        let (row_id, col_id) = *cell;

        if row_id > 0 {
            let top = (row_id - 1, col_id);
            let top_val = self.cave[top.0][top.1];
            neighbors.push((top, top_val));
        }
        if col_id < N - 1 {
            let right = (row_id, col_id + 1);
            let right_val = self.cave[right.0][right.1];
            neighbors.push((right, right_val));
        }
        if col_id > 0 {
            let left = (row_id, col_id - 1);
            let left_val = self.cave[left.0][left.1];
            neighbors.push((left, left_val));
        }
        if row_id < N - 1 {
            let bottom = (row_id + 1, col_id);
            let bottom_val = self.cave[bottom.0][bottom.1];
            neighbors.push((bottom, bottom_val));
        }
        neighbors
    }
}

fn main() {
    let filepath = "day15.in";
    let data = match fs::read_to_string(filepath) {
        Ok(data) => data,
        Err(why) => panic!("Weird: {}", why),
    };
    let cave_map: CaveMap<100> = CaveMap::from_str(&data[..]).unwrap();
    // let cave_map: CaveMap<10> = CaveMap::from(data);
    println!("{}", cave_map);
    let ans1 = cave_map.shortest_path_to_bottom();
    println!("{}", ans1);
}

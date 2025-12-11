advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, 1000)
}

pub fn part_two(input: &str) -> Option<u64> {
    let positions = parse_input(input);
    if positions.is_empty() {
        return None;
    }

    let mut edges = Vec::new();
    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            let dist = distance(&positions[i], &positions[j]);
            edges.push((dist, i, j));
        }
    }

    edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut uf = UnionFind::new(positions.len());

    for &(_, u, v) in &edges {
        let before = uf.count_components();
        uf.union(u, v);
        let after = uf.count_components();

        if after == 1 && before > 1 {
            let x1 = positions[u].0 as u64;
            let x2 = positions[v].0 as u64;
            return Some(x1 * x2);
        }
    }

    None
}

fn solve(input: &str, num_connections: usize) -> Option<u64> {
    let positions = parse_input(input);
    if positions.is_empty() {
        return None;
    }

    let mut edges = Vec::new();
    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            let dist = distance(&positions[i], &positions[j]);
            edges.push((dist, i, j));
        }
    }

    edges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut uf = UnionFind::new(positions.len());
    for &(_, u, v) in edges.iter().take(num_connections) {
        uf.union(u, v);
    }

    let circuit_sizes = uf.get_component_sizes();
    let result = find_largest(&circuit_sizes)?;

    Some(result)
}

type Position = (i64, i64, i64);

fn parse_input(input: &str) -> Vec<Position> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 3 {
                let x = parts[0].trim().parse().ok()?;
                let y = parts[1].trim().parse().ok()?;
                let z = parts[2].trim().parse().ok()?;
                Some((x, y, z))
            } else {
                None
            }
        })
        .collect()
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            // Union by size
            if self.size[root_x] < self.size[root_y] {
                self.parent[root_x] = root_y;
                self.size[root_y] += self.size[root_x];
            } else {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
            }
        }
    }

    fn get_component_sizes(&mut self) -> Vec<usize> {
        let mut sizes = std::collections::HashMap::new();
        for i in 0..self.parent.len() {
            let root = self.find(i);
            *sizes.entry(root).or_insert(0) += 1;
        }
        sizes.values().copied().collect()
    }

    fn count_components(&mut self) -> usize {
        let mut roots = std::collections::HashSet::new();
        for i in 0..self.parent.len() {
            roots.insert(self.find(i));
        }
        roots.len()
    }
}

fn distance(a: &Position, b: &Position) -> f64 {
    let dx = (a.0 - b.0) as f64;
    let dy = (a.1 - b.1) as f64;
    let dz = (a.2 - b.2) as f64;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

fn find_largest(sizes: &[usize]) -> Option<u64> {
    if sizes.len() < 3 {
        return None;
    }

    let mut sorted = sizes.to_vec();
    sorted.sort_unstable_by(|a, b| b.cmp(a)); // Sort descending

    Some(sorted[0] as u64 * sorted[1] as u64 * sorted[2] as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let result = solve(&input, 10);
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}

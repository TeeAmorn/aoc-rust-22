use std::collections::{HashMap, BinaryHeap, HashSet};

pub fn solve(input: &Vec<String>) -> String {
    let rows = input.len();
    let cols = input[0].len();

    // Convert input to 2D array of chars
    let char_map = input.iter().map(|line| {
        line.chars().collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    // Convert array of chars to integers
    let mut graph = HashMap::new();
    let mut start = vec![];
    let mut end = (0, 0);
    let mut map = vec![vec![0; cols]; rows];
    for i in 0..rows {
        for j in 0..cols {
            let c = char_map[i][j];
            map[i][j] = match c {
                'S' | 'a' => {
                    start.push((i, j));
                    0
                }
                'E' => {
                    end = (i, j);
                    25
                }
                _ => {
                    c as i8 - 97
                }
            };
            graph.insert((i, j), vec![]);
        }
    }

    // Construct graph
    for i in 0..rows {
        for j in 0..cols {
            if j < cols - 1 && map[i][j] - 1 <= map[i][j + 1] {
                graph.get_mut(&(i, j + 1)).unwrap().push((i, j));
            }
            if i < rows - 1 && map[i][j] - 1 <= map[i + 1][j] {
                graph.get_mut(&(i + 1, j)).unwrap().push((i, j));
            }
            if j < cols - 1 && map[i][j] >= map[i][j + 1] - 1 {
                graph.get_mut(&(i, j)).unwrap().push((i, j + 1));
            }
            if i < rows - 1 && map[i][j] >= map[i + 1][j] - 1 {
                graph.get_mut(&(i, j)).unwrap().push((i + 1, j));
            }
        }
    }

    // Find minimum distance
    let mut answer = i32::MAX;
    for src in start {
        answer = std::cmp::min(answer, get_shortest(&graph, src, end));
    }
    
    answer.to_string()
}

fn get_shortest(graph: &HashMap<(usize, usize), Vec<(usize, usize)>>, src: (usize, usize), dst: (usize, usize)) -> i32 {
    let mut dist = HashMap::new();
    let mut to_do = BinaryHeap::new();
    let mut done = HashSet::new();

    for pos in graph.keys() {
        dist.insert(pos, i32::MAX);
    }
    dist.insert(&src, 0);
    to_do.push((0, src));

    while let Some((d, u)) = to_do.pop() {
        if done.contains(&u) {
            continue;
        }
        done.insert(u);
        for neighbor in graph[&u].iter() {
            if dist[neighbor] > dist[&u] + 1 {
                *dist.get_mut(neighbor).unwrap() = -d + 1;
                to_do.push((-dist[neighbor], neighbor.clone()));
            }
        }
    }

    dist[&dst]
}

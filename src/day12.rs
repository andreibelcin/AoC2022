use std::{collections::VecDeque, str::FromStr};

use itertools::Itertools;

const DIRECTION_DELTAS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

struct HeightMap {
    inner: Vec<Vec<u8>>,
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
}

impl HeightMap {
    fn find_path_length_from_S_to_E(&self) -> Result<u32, ()> {
        let mut visited = vec![vec![false; self.width]; self.height];
        let mut q = VecDeque::new();
        q.push_back((self.start, 0));
        visited[self.start.0][self.start.1] = true;
        while !q.is_empty() {
            let (point, dist) = q.pop_front().unwrap();
            if point == self.end {
                return Ok(dist);
            }
            for delta in DIRECTION_DELTAS {
                let neighbour = (point.0 as i32 + delta.0, point.1 as i32 + delta.1);
                if 0 <= neighbour.0
                    && neighbour.0 < self.height as i32
                    && 0 <= neighbour.1
                    && neighbour.1 < self.width as i32
                {
                    let neighbour = (neighbour.0 as usize, neighbour.1 as usize);
                    if !visited[neighbour.0][neighbour.1]
                        && self.inner[neighbour.0][neighbour.1] <= self.inner[point.0][point.1] + 1
                    {
                        q.push_back((neighbour, dist + 1));
                        visited[neighbour.0][neighbour.1] = true;
                    }
                }
            }
        }
        Err(())
    }

    fn find_path_length_from_E_to_a(&self) -> Result<u32, ()> {
        let mut visited = vec![vec![false; self.width]; self.height];
        let mut q = VecDeque::new();
        q.push_back((self.end, 0));
        visited[self.end.0][self.end.1] = true;
        while !q.is_empty() {
            let (point, dist) = q.pop_front().unwrap();
            if self.inner[point.0][point.1] == 0 {
                return Ok(dist);
            }
            for delta in DIRECTION_DELTAS {
                let neighbour = (point.0 as i32 + delta.0, point.1 as i32 + delta.1);
                if 0 <= neighbour.0
                    && neighbour.0 < self.height as i32
                    && 0 <= neighbour.1
                    && neighbour.1 < self.width as i32
                {
                    let neighbour = (neighbour.0 as usize, neighbour.1 as usize);
                    if !visited[neighbour.0][neighbour.1]
                        && self.inner[point.0][point.1] <= self.inner[neighbour.0][neighbour.1] + 1
                    {
                        q.push_back((neighbour, dist + 1));
                        visited[neighbour.0][neighbour.1] = true;
                    }
                }
            }
        }
        Err(())
    }
}

impl FromStr for HeightMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = None;
        let mut end = None;
        let inner = s
            .lines()
            .enumerate()
            .map(|(j, l)| {
                l.chars()
                    .enumerate()
                    .map(|(i, c)| match c {
                        'S' => {
                            start = Some((j, i));
                            0
                        }
                        'E' => {
                            end = Some((j, i));
                            25
                        }
                        o => o as u8 - 'a' as u8,
                    })
                    .collect_vec()
            })
            .collect_vec();

        let height = inner.len();
        let width = inner[0].len();

        Ok(Self {
            inner,
            width,
            height,
            start: start.unwrap(),
            end: end.unwrap(),
        })
    }
}

pub fn solve(input: String) -> (String, String) {
    let height_map = input.parse::<HeightMap>().unwrap();
    let result1 = height_map.find_path_length_from_S_to_E().unwrap();
    let result2 = height_map.find_path_length_from_E_to_a().unwrap();

    (
        format!("Path length: {result1}"),
        format!("Scenic path length: {result2}"),
    )
}

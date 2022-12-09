use std::{collections::HashSet, str::FromStr};

struct TreeGrid {
    vec: Vec<Vec<u32>>,
    n: usize,
    m: usize,
}

impl TreeGrid {
    fn count_visible_trees(&self) -> usize {
        let mut visible_trees = HashSet::new();
        for i in 0..self.n {
            let mut tallest_tree = self.vec[i][0];
            visible_trees.insert((i, 0));
            for j in 1..self.m {
                if self.vec[i][j] > tallest_tree {
                    tallest_tree = self.vec[i][j];
                    visible_trees.insert((i, j));
                }
            }
            tallest_tree = self.vec[i][self.m - 1];
            visible_trees.insert((i, self.m - 1));
            for j in (0..(self.m - 1)).rev() {
                if self.vec[i][j] > tallest_tree {
                    tallest_tree = self.vec[i][j];
                    visible_trees.insert((i, j));
                }
            }
        }
        for j in 0..self.m {
            let mut tallest_tree = self.vec[0][j];
            visible_trees.insert((0, j));
            for i in 1..self.n {
                if self.vec[i][j] > tallest_tree {
                    tallest_tree = self.vec[i][j];
                    visible_trees.insert((i, j));
                }
            }
            tallest_tree = self.vec[self.n - 1][j];
            visible_trees.insert((self.n - 1, j));
            for i in (0..(self.n - 1)).rev() {
                if self.vec[i][j] > tallest_tree {
                    tallest_tree = self.vec[i][j];
                    visible_trees.insert((i, j));
                }
            }
        }
        visible_trees.len()
    }

    fn find_best_scenic_score(&self) -> u32 {
        let mut tallest_trees = HashSet::new();
        for i in 0..self.n {
            for j in 0..self.m {
                tallest_trees.insert((i, j));
            }
        }
        // for i in 0..self.n {
        //     let mut tallest_tree_idx = (i, 0);
        //     for j in 0..self.m {
        //         if self.vec[i][j] > self.vec[tallest_tree_idx.0][tallest_tree_idx.1] {
        //             tallest_tree_idx = (i, j);
        //         }
        //     }
        //     tallest_trees.insert(tallest_tree_idx);
        // }
        // for j in 0..self.m {
        //     let mut tallest_tree_idx = (0, j);
        //     for i in 0..self.n {
        //         if self.vec[i][j] > self.vec[tallest_tree_idx.0][tallest_tree_idx.1] {
        //             tallest_tree_idx = (i, j);
        //         }
        //     }
        //     tallest_trees.insert(tallest_tree_idx);
        // }

        let mut max_score = 1;
        for tree_idx in tallest_trees {
            let mut tree_score = 1;
            let mut count = 0;
            'a: for i in (0..tree_idx.0).rev() {
                count += 1;
                if self.vec[i][tree_idx.1] >= self.vec[tree_idx.0][tree_idx.1] {
                    break 'a;
                }
            }
            tree_score *= count;
            count = 0;
            'b: for i in (tree_idx.0 + 1)..self.n {
                count += 1;
                if self.vec[i][tree_idx.1] >= self.vec[tree_idx.0][tree_idx.1] {
                    break 'b;
                }
            }
            tree_score *= count;
            count = 0;
            'c: for j in (0..tree_idx.1).rev() {
                count += 1;
                if self.vec[tree_idx.0][j] >= self.vec[tree_idx.0][tree_idx.1] {
                    break 'c;
                }
            }
            tree_score *= count;
            count = 0;
            'd: for j in (tree_idx.1 + 1)..self.m {
                count += 1;
                if self.vec[tree_idx.0][j] >= self.vec[tree_idx.0][tree_idx.1] {
                    break 'd;
                }
            }
            tree_score *= count;

            if max_score < tree_score {
                max_score = tree_score;
            }
        }
        max_score
    }
}

impl FromStr for TreeGrid {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec: Vec<Vec<u32>> = s
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        let n = vec.len();
        let m = vec[0].len();
        Ok(Self { vec, n, m })
    }
}

pub fn solve(input: String) -> (String, String) {
    let tree_grid = input.parse::<TreeGrid>().unwrap();
    let result1 = tree_grid.count_visible_trees();
    let result2 = tree_grid.find_best_scenic_score();

    (
        format!("Visible tree count: {result1}"),
        format!("Best scenic score: {result2}"),
    )
}

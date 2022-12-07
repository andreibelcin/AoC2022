use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

enum Command {
    Cd { path: String },
    Ls { contents: Vec<LsOutput> },
}

enum LsOutput {
    Dir,
    File { size: u32 },
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[..2] {
            "cd" => Ok(Command::Cd {
                path: s[3..].to_owned(),
            }),
            "ls" => Ok(Command::Ls {
                contents: s[3..].lines().map(|l| l.parse().unwrap()).collect_vec(),
            }),
            _ => panic!("Unknown command: {s}"),
        }
    }
}

impl FromStr for LsOutput {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(" ").unwrap() {
            ("dir", _) => Ok(LsOutput::Dir),
            (size, _) => Ok(LsOutput::File {
                size: size.parse().unwrap(),
            }),
        }
    }
}

fn get_dir_sizes(instructions: Vec<Command>) -> HashMap<String, u32> {
    let mut dir_sizes = HashMap::<String, u32>::new();
    let mut dir_stack = Vec::new();

    for instruction in instructions {
        match instruction {
            Command::Cd { path } => {
                if path == ".." {
                    dir_stack.pop();
                } else if path == "/" {
                    dir_stack.clear();
                } else {
                    dir_stack.push(path)
                }
            }
            Command::Ls { contents } => {
                let total_file_size = contents
                    .iter()
                    .map(|o| match o {
                        LsOutput::Dir => 0,
                        LsOutput::File { size } => *size,
                    })
                    .sum::<u32>();

                let mut path = String::from("/");
                dir_sizes.insert(
                    path.clone(),
                    dir_sizes.get(&path).unwrap_or(&0) + total_file_size,
                );

                for dir in &dir_stack {
                    path += &(dir.to_string() + "/");
                    dir_sizes.insert(
                        path.clone(),
                        dir_sizes.get(&path).unwrap_or(&0) + total_file_size,
                    );
                }
            }
        };
    }

    dir_sizes
}

pub fn solve(input: String) -> (String, String) {
    let instructions = input[2..]
        .split("\n$ ")
        .map(|s| s.parse().unwrap())
        .collect_vec();

    let dir_sizes = get_dir_sizes(instructions);

    let result1 = dir_sizes
        .values()
        .filter_map(|&val| if val <= 100000 { Some(val) } else { None })
        .sum::<u32>();

    const TOTAL_MEM: u32 = 70000000;
    const MIN_MEM: u32 = 30000000;
    let used_mem = dir_sizes["/"];
    let needed_mem = MIN_MEM - (TOTAL_MEM - used_mem);
    let result2 = *dir_sizes
        .values()
        .sorted()
        .find(|&&s| s >= needed_mem)
        .unwrap();

    (
        format!("Total size of dirs below 100000: {result1}"),
        format!("Directory size to be deleted: {result2}"),
    )
}

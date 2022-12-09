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

#[allow(dead_code)]
mod FileSystemMethod {
    use std::{
        cell::RefCell,
        collections::HashMap,
        rc::{Rc, Weak},
    };

    #[derive(Debug)]
    enum FileSystem {
        File(File),
        Dir(Rc<Dir>),
    }

    #[derive(Debug)]
    struct File {
        size: u32,
    }

    #[derive(Debug)]
    struct Dir {
        parent: Option<Weak<Dir>>,
        content: RefCell<HashMap<String, FileSystem>>,
    }

    impl FileSystem {
        fn from_instructions(instructions: Vec<&str>) -> FileSystem {
            let root = Rc::new(Dir {
                parent: None,
                content: RefCell::new(HashMap::new()),
            });

            let mut current_dir = root.clone();
            for instruction in instructions {
                match &instruction[..2] {
                    "cd" => {
                        current_dir = match &instruction[3..] {
                            "/" => root.clone(),
                            ".." => current_dir.parent.as_ref().unwrap().upgrade().unwrap(),
                            dir_name => {
                                if let FileSystem::Dir(dir) =
                                    &current_dir.content.borrow()[dir_name]
                                {
                                    dir.clone()
                                } else {
                                    panic!("Not a directory: {dir_name}")
                                }
                            }
                        }
                    }
                    "ls" => {
                        for line in instruction[3..].lines() {
                            match line.split_once(" ").unwrap() {
                                ("dir", dir_name) => current_dir.content.borrow_mut().insert(
                                    dir_name.to_owned(),
                                    FileSystem::Dir(Rc::new(Dir {
                                        parent: Some(Rc::downgrade(&current_dir)),
                                        content: RefCell::new(HashMap::new()),
                                    })),
                                ),
                                (size, file_name) => current_dir.content.borrow_mut().insert(
                                    file_name.to_owned(),
                                    FileSystem::File(File {
                                        size: size.parse().unwrap(),
                                    }),
                                ),
                            };
                        }
                    }
                    _ => panic!("Unknown instruction: {instruction}"),
                }
            }

            FileSystem::Dir(root.clone())
        }

        fn size(&self) -> u32 {
            match self {
                FileSystem::File(file) => file.size,
                FileSystem::Dir(dir) => dir.content.borrow().values().map(|v| v.size()).sum(),
            }
        }
    }
}

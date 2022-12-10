use std::{fmt::Display, str::FromStr};

use itertools::Itertools;

enum Instruction {
    NOOP,
    ADDX(i32),
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[..4] {
            "noop" => Ok(Self::NOOP),
            "addx" => Ok(Self::ADDX(s[5..].parse().unwrap())),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct CPU {
    clock: u32,
    register: i32,
}

impl CPU {
    fn new() -> CPU {
        CPU {
            clock: 0,
            register: 1,
        }
    }

    fn execute<F>(&mut self, instructions: &Vec<Instruction>, mut debug: F)
    where
        F: FnMut(&Self) -> (),
    {
        for instruction in instructions {
            self.clock += 1;
            debug(self);
            if let Instruction::ADDX(x) = instruction {
                self.clock += 1;
                debug(self);
                self.register += x;
            }
        }
    }
}

struct CRT {
    cpu: CPU,
    screen: [bool; 40 * 6],
}

impl CRT {
    fn new() -> CRT {
        CRT {
            cpu: CPU::new(),
            screen: [false; 40 * 6],
        }
    }

    fn execute(&mut self, instructions: &Vec<Instruction>) {
        self.cpu.execute(instructions, |cpu| {
            self.screen[cpu.clock as usize - 1] =
                cpu.register.abs_diff(((cpu.clock - 1) % 40) as i32) <= 1
        })
    }
}

impl Display for CRT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for i in 0..6 {
            s += &self.screen[(40 * i)..(40 * (i + 1))]
                .iter()
                .map(|b| if *b { '#' } else { ' ' })
                .collect::<String>();
            s += "\n";
        }

        f.write_str(&s)
    }
}

pub fn solve(input: String) -> (String, String) {
    let instructions = input
        .lines()
        .map(|l| l.parse::<Instruction>().unwrap())
        .collect_vec();

    let mut result1 = 0;
    let mut CPU = CPU::new();
    CPU.execute(&instructions, |cpu| {
        if cpu.clock >= 20 && (cpu.clock - 20) % 40 == 0 {
            result1 += cpu.register * cpu.clock as i32
        }
    });

    let mut crt = CRT::new();
    crt.execute(&instructions);

    (
        format!("Signal strength: {result1}"),
        format!("Screen:\n{}", crt),
    )
}

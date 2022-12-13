use std::{cmp::Ordering, str::FromStr};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Val(u32),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Val(x), Packet::Val(y)) => x.cmp(y),
            (Packet::Val(x), Packet::List(_)) => Packet::List(vec![Packet::Val(*x)]).cmp(other),
            (Packet::List(_), Packet::Val(y)) => self.cmp(&Packet::List(vec![Packet::Val(*y)])),
            (Packet::List(self_v), Packet::List(other_v)) => {
                let n = self_v.len();
                let m = other_v.len();
                for i in 0..(n.min(m)) {
                    if self_v[i] == other_v[i] {
                        continue;
                    }
                    return self_v[i].cmp(&other_v[i]);
                }
                n.cmp(&m)
            }
        }
    }
}

impl FromStr for Packet {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.chars().collect_vec();
        let mut stack = Vec::new();
        let mut current_vec = Vec::new();
        let mut i = 1;
        while i < (s.len() - 1) {
            if s[i] == '[' {
                stack.push(current_vec);
                current_vec = Vec::new();
            } else if s[i] == ']' {
                let mut previous_vec = stack.pop().unwrap();
                previous_vec.push(Packet::List(current_vec));
                current_vec = previous_vec;
            } else if s[i].is_ascii_digit() {
                let mut j = i;
                while s[j].is_ascii_digit() {
                    j += 1;
                }
                current_vec.push(Packet::Val(s[i..j].iter().join("").parse::<u32>().unwrap()));
                i = j - 1;
            }
            i += 1;
        }
        Ok(Packet::List(current_vec))
    }
}

pub fn solve(input: String) -> (String, String) {
    let packet_pairs = input
        .split("\n\n")
        .map(|ls| {
            ls.lines()
                .map(|l| l.parse::<Packet>().unwrap())
                .collect_tuple::<(Packet, Packet)>()
                .unwrap()
        })
        .collect_vec();

    let result1 = packet_pairs
        .iter()
        .enumerate()
        .filter(|(_, p)| p.0 <= p.1)
        .map(|(i, _)| i + 1)
        .sum::<usize>();

    let packet_2 = Packet::List(vec![Packet::List(vec![Packet::Val(2)])]);
    let packet_6 = Packet::List(vec![Packet::List(vec![Packet::Val(6)])]);

    let packet_2_count = packet_pairs
        .iter()
        .flat_map(|pp| vec![&pp.0, &pp.1])
        .filter(|p| *p < &packet_2)
        .count()
        + 1;
    let packet_6_count = packet_pairs
        .iter()
        .flat_map(|pp| vec![&pp.0, &pp.1])
        .filter(|p| *p < &packet_6)
        .count()
        + 2;

    let result2 = packet_2_count * packet_6_count;

    (
        format!("Sum of pairs in the right order: {result1}"),
        format!("Decoder key: {result2}"),
    )
}

use itertools::Itertools;

fn char_to_priority(c: char) -> u32 {
    c as u32
        - if c.is_ascii_lowercase() {
            'a' as u32 - 1
        } else {
            'A' as u32 - 27
        }
}

pub fn solve(input: String) -> (String, String) {
    let result1: u32 = input
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|p| p.0.chars().find(|c| p.1.contains(*c)).unwrap())
        .map(|c| char_to_priority(c))
        .sum();

    let result2: u32 = input
        .lines()
        .tuples::<(_, _, _)>()
        .map(|t| {
            t.0.chars()
                .find(|c| t.1.contains(*c) && t.2.contains(*c))
                .unwrap()
        })
        .map(|c| char_to_priority(c))
        .sum();

    (
        format!("Sum of priorities: {result1}"),
        format!("Sum of badge priorities: {result2}"),
    )
}

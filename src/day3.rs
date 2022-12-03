use itertools::Itertools;

fn priority_of(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => panic!("{c} is not a letter"),
    }
}

pub fn solve(input: String) -> (String, String) {
    let result1: u32 = input
        .lines()
        .map(|l| l.split_at(l.len() / 2))
        .map(|p| p.0.chars().find(|c| p.1.contains(*c)).unwrap())
        .map(|c| priority_of(c))
        .sum();

    let result2: u32 = input
        .lines()
        .tuples::<(_, _, _)>()
        .map(|t| {
            t.0.chars()
                .find(|c| t.1.contains(*c) && t.2.contains(*c))
                .unwrap()
        })
        .map(|c| priority_of(c))
        .sum();

    (
        format!("Sum of priorities: {result1}"),
        format!("Sum of badge priorities: {result2}"),
    )
}

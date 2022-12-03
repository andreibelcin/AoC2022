use itertools::Itertools;

pub fn solve(input: String) -> (String, String) {
    let input: Vec<i32> = input
        .split("\n\n")
        .map(|s| s.lines().map(|l| l.parse::<i32>().unwrap()).sum())
        .collect();

    let max_calories_1 = input.iter().max().unwrap();
    let max_calories_2 = input.iter().sorted().rev().take(3).sum::<i32>();

    (
        format!("Top calories: {max_calories_1}"),
        format!("Top calories 2: {max_calories_2}"),
    )
}

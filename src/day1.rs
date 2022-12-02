use itertools::Itertools;

pub fn solve(input: String) -> (String, String) {
    let input: Vec<Vec<i32>> = input
        .split("\n\n")
        .map(|s| s.lines().map(|l| l.parse::<i32>().unwrap()).collect())
        .collect();

    let max_calories_1 = input.iter().map(|v| v.iter().sum::<i32>()).max().unwrap();
    let max_calories_2 = input
        .iter()
        .map(|v| v.iter().sum::<i32>())
        .sorted()
        .rev()
        .take(3)
        .sum::<i32>();

    (
        format!("Top calories: {max_calories_1}"),
        format!("Top calories 2: {max_calories_2}"),
    )
}

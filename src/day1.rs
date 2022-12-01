use itertools::Itertools;

pub fn solve(input: String) {
    let input: Vec<Vec<i32>> = input
        .split("\n\n")
        .map(|s| s.lines().map(|l| l.parse::<i32>().unwrap()).collect())
        .collect();

    let max_calories = input.iter().map(|v| v.iter().sum::<i32>()).max().unwrap();
    println!("Top calories: {max_calories}");

    let max_3_calories = input
        .iter()
        .map(|v| v.iter().sum::<i32>())
        .sorted()
        .rev()
        .take(3)
        .sum::<i32>();
    println!("Top 3 calories: {max_3_calories}");
}

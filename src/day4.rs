struct SectionRange(i32, i32);

impl SectionRange {
    fn includes(&self, other: &SectionRange) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }

    fn overlaps(&self, other: &SectionRange) -> bool {
        self.contains(other.0) || other.contains(self.0)
    }

    fn contains(&self, x: i32) -> bool {
        self.0 <= x && x <= self.1
    }
}

impl From<(&str, &str)> for SectionRange {
    fn from(s: (&str, &str)) -> Self {
        Self(s.0.parse().unwrap(), s.1.parse().unwrap())
    }
}

impl From<&str> for SectionRange {
    fn from(s: &str) -> Self {
        s.split_once("-").unwrap().into()
    }
}

pub fn solve(input: String) -> (String, String) {
    let ranges: Vec<(SectionRange, SectionRange)> = input
        .lines()
        .map(|l| l.split_once(",").unwrap())
        .map(|s| (s.0.into(), s.1.into()))
        .collect();

    let count1 = ranges
        .iter()
        .filter(|r| r.0.includes(&r.1) || r.1.includes(&r.0))
        .count();

    let count2 = ranges.iter().filter(|r| r.0.overlaps(&r.1)).count();

    (
        format!("Number of fully contained sections: {count1}"),
        format!("Number of overlapping sections: {count2}"),
    )
}

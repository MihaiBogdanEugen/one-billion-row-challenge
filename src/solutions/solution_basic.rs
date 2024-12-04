use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::fmt::Write;
use std::fs::read_to_string;

use crate::utils::rounding::round_one_digit_precision;

use super::solver::Solver;

#[derive(Default)]
pub struct SolutioBasic {}

impl Solver for SolutioBasic {
    fn solve_obrc(input_path: &str) -> String {
        let input: String = read_to_string(input_path).unwrap();
        let mut output: String = String::new();
        let mut map: HashMap<&str, Statistics> = HashMap::<&str, Statistics>::default();

        for line in input.lines() {
            if let Some((name, temperature)) = parse_line(line) {
                map.entry(name)
                    .and_modify(|stats: &mut Statistics| {
                        stats.min = stats.min.min(temperature);
                        stats.max = stats.max.max(temperature);
                        stats.count += 1;
                        stats.mean = (stats.mean * (stats.count - 1) as f64 + temperature)
                            / stats.count as f64;
                    })
                    .or_insert(Statistics {
                        min: temperature,
                        max: temperature,
                        mean: temperature,
                        count: 1,
                    });
            }
        }

        for (name, stats) in map {
            writeln!(output, "{}={}", name, stats).unwrap();
        }

        output
    }
}

fn parse_line(line: &str) -> Option<(&str, f64)> {
    let (name, temperature_as_str) = line.split_once(';')?;
    let temperature: f64 = temperature_as_str.parse::<f64>().ok()?;
    Some((name, temperature))
}

#[derive(Default)]
struct Statistics {
    min: f64,
    max: f64,
    mean: f64,
    count: u64,
}

impl Display for Statistics {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}/{}/{}",
            round_one_digit_precision(self.min),
            round_one_digit_precision(self.max),
            round_one_digit_precision(self.mean)
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::fs::read_to_string;

    use super::SolutioBasic;
    use super::Solver;

    #[test]
    fn test_solution_1000000() {
        let expected: HashSet<String> = read_to_string("resources/results_1000000.txt")
            .unwrap()
            .lines()
            .filter(|line: &&str| !line.is_empty())
            .map(String::from)
            .collect();

        let actual: HashSet<String> =
            SolutioBasic::solve_obrc("resources/measurements_1000000.txt")
                .lines()
                .filter(|line: &&str| !line.is_empty())
                .map(String::from)
                .collect();

        assert_eq!(expected.len(), actual.len());
        expected
            .iter()
            .for_each(|expected_line: &String| assert!(actual.contains(expected_line)));
    }
}

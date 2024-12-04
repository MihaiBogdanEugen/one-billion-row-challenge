use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::fmt::Write;
use std::fs::read_to_string;

use super::solver::Solver;
use crate::utils::parsing::to;
use crate::utils::rounding::round_one_digit_precision;

#[derive(Default)]
pub struct SolutioNaive {}

impl Solver for SolutioNaive {
    fn solve_obrc(input_path: &str) -> String {
        let input: String = read_to_string(input_path).unwrap();
        let mut output: String = String::new();

        input
            .lines()
            .map(|line: &str| -> (&str, i64) {
                line.split_once(';')
                    .map(|(name, temperature_as_str)| {
                        (name, (to::<f64>(temperature_as_str) * 10.0) as i64)
                    })
                    .unwrap()
            })
            .fold(
                HashMap::<&str, Statistics>::default(),
                |mut acc: HashMap<&str, Statistics>,
                 (name, temperature): (&str, i64)|
                 -> HashMap<&str, Statistics> {
                    let stats: &mut Statistics = acc.entry(name).or_default();
                    stats.update(temperature);
                    acc
                },
            )
            .iter_mut()
            .map(|(name, stats)| {
                stats.compute();
                (name, stats)
            })
            .for_each(|(name, stats)| {
                writeln!(output, "{}={}", name, stats).unwrap();
            });

        output
    }
}

struct Statistics {
    curr_min: i64,
    curr_max: i64,
    acc_sum: i64,
    count: u64,
    min: f64,
    max: f64,
    mean: f64,
}

impl Statistics {
    fn update(&mut self, temperature: i64) {
        self.curr_min = self.curr_min.min(temperature);
        self.curr_max = self.curr_max.max(temperature);
        self.acc_sum += temperature;
        self.count += 1;
    }

    fn compute(&mut self) {
        self.min = round_one_digit_precision(self.curr_min as f64 / 10.0);
        self.max = round_one_digit_precision(self.curr_max as f64 / 10.0);
        self.mean = round_one_digit_precision(self.acc_sum as f64 / (10.0 * self.count as f64));
    }
}

impl Default for Statistics {
    fn default() -> Self {
        Self {
            curr_min: i64::MAX,
            curr_max: i64::MIN,
            acc_sum: 0,
            count: 0,
            max: 0.0,
            min: 0.0,
            mean: 0.0,
        }
    }
}

impl Display for Statistics {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}/{}/{}", self.min, self.max, self.mean)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::fs::read_to_string;

    use super::SolutioNaive;
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
            SolutioNaive::solve_obrc("resources/measurements_1000000.txt")
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

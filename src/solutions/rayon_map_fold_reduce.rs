use crate::utils::hash::FastHashMap;
use crate::utils::parsing::to;
use crate::utils::rounding::round_one_digit_precision;
use rayon::prelude::*;

pub fn solve(input: &str) -> FastHashMap<&str, Statistics> {
    let mut result: FastHashMap<&str, Statistics> = input
        .par_lines()
        //.filter(|line: &&str| -> bool { !line.is_empty() })
        .map(|line: &str| -> (&str, i64) {
            line.split_once(';')
                .map(|(name, temperature_as_str)| {
                    (name, (to::<f64>(temperature_as_str) * 10.0) as i64)
                })
                .unwrap()
        })
        .fold(
            FastHashMap::<&str, Statistics>::default,
            |mut acc: FastHashMap<&str, Statistics>,
             (name, temperature): (&str, i64)|
             -> FastHashMap<&str, Statistics> {
                let stats: &mut Statistics = acc.entry(name).or_default();
                stats.update(temperature);
                acc
            },
        )
        .reduce(
            FastHashMap::<&str, Statistics>::default,
            |mut acc: FastHashMap<&str, Statistics>,
             map: FastHashMap<&str, Statistics>|
             -> FastHashMap<&str, Statistics> {
                for (name, stats) in map {
                    let acc_stats: &mut Statistics = acc.entry(name).or_default();
                    acc_stats.merge(&stats);
                }
                acc
            },
        );
    result
        .values_mut()
        .for_each(|stats: &mut Statistics| stats.compute());
    result
}

pub struct Statistics {
    curr_min: i64,
    curr_max: i64,
    acc_sum: i64,
    count: u64,
    pub min: f64,
    pub max: f64,
    pub mean: f64,
}

impl Statistics {
    fn update(&mut self, temperature: i64) {
        self.curr_min = self.curr_min.min(temperature);
        self.curr_max = self.curr_max.max(temperature);
        self.acc_sum += temperature;
        self.count += 1;
    }

    fn merge(&mut self, other: &Statistics) {
        self.curr_min = self.curr_min.min(other.curr_min);
        self.curr_max = self.curr_max.max(other.curr_max);
        self.acc_sum += other.acc_sum;
        self.count += other.count;
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

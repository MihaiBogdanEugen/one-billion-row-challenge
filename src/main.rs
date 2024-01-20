use obrc::util::hash::FastMap;
use obrc::util::parsing::to;
use obrc::util::rounding::round_one_digit_precision;
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::env::args;
use std::fs::read_to_string;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::PathBuf;
use std::time::Duration;
use std::time::Instant;

struct Statistics {
    curr_min: i64,
    curr_max: i64,
    acc_sum: i64,
    count: usize,
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

fn main() {
    let path_str: String = args()
        .nth(1)
        .unwrap_or(String::from("measurements_1000000000.txt"));
    let path: PathBuf = PathBuf::from(&path_str);
    assert!(path.exists(), "Path {:?} does not exist!", path);

    let input: String = read_to_string(path).unwrap();

    let filter_op = |line: &&str| -> bool { !line.is_empty() };

    let map_op = |line: &str| -> (String, i64) {
        line.split_once(';')
            .map(|(name_as_str, temperature_as_str)| {
                (
                    name_as_str.to_owned(),
                    (to::<f64>(temperature_as_str) * 10.0) as i64,
                )
            })
            .unwrap()
    };

    let fold_op = |mut acc: FastMap<String, Statistics>,
                   (name, temperature): (String, i64)|
     -> FastMap<String, Statistics> {
        let stats: &mut Statistics = acc.entry(name.to_string()).or_default();
        stats.update(temperature);
        acc
    };

    let reduce_op = |mut acc: FastMap<String, Statistics>,
                     map: FastMap<String, Statistics>|
     -> FastMap<String, Statistics> {
        for (name, stats) in map {
            let acc_stats: &mut Statistics = acc.entry(name).or_default();
            acc_stats.merge(&stats);
        }
        acc
    };

    let identity = FastMap::<String, Statistics>::default;

    let now: Instant = Instant::now();

    let mut result: FastMap<String, Statistics> = input
        .par_lines()
        .filter(filter_op)
        .map(map_op)
        .fold(identity, fold_op)
        .reduce(identity, reduce_op);
    result
        .values_mut()
        .for_each(|stats: &mut Statistics| stats.compute());

    let duration: Duration = now.elapsed();
    println!("Results for {} generated in {:?}", &path_str, duration);

    let file: File = File::create(path_str.replace("measurements", "results"))
        .unwrap_or_else(|_| panic!("Cannot create results fine!"));
    let mut writer: BufWriter<File> = BufWriter::new(file);

    for (name, stats) in BTreeMap::from_par_iter(result.into_par_iter()) {
        writeln!(
            writer,
            "{}={}/{}/{}",
            name, stats.min, stats.max, stats.mean
        )
        .unwrap();
    }
}

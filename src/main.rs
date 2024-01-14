use clap::Parser;
use rayon::iter::ParallelIterator;
use rayon::prelude::*;
use rustc_hash::FxHashMap;
use std::collections::BTreeMap;
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

fn round_one_digit_precision(x: f64) -> f64 {
    format!("{:.1$}", x, 1).parse::<f64>().unwrap()
}

impl Statistics {
    fn update(&mut self, temperature: i64) {
        self.curr_min = self.curr_min.min(temperature);
        self.curr_max = self.curr_max.max(temperature);
        self.acc_sum += temperature;
        self.count += 1;
    }

    fn merge(&mut self, other: Statistics) {
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

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    path: String,
}

fn main() {
    let cli: Cli = Cli::parse();
    let path: PathBuf = PathBuf::from(&cli.path);
    let input: String = std::fs::read_to_string(path).unwrap();

    let filter_op = |line: &&str| -> bool { !line.is_empty() };

    let map_op = |line: &str| -> (String, i64) {
        line.split_once(';')
            .map(|(name_as_str, temperature_as_str)| {
                (
                    name_as_str.to_owned(),
                    (temperature_as_str.parse::<f64>().unwrap() * 10.0) as i64,
                )
            })
            .unwrap()
    };

    let fold_op = |mut acc: FxHashMap<String, Statistics>,
                   (name, temperature): (String, i64)|
     -> FxHashMap<String, Statistics> {
        let stats: &mut Statistics = acc.entry(name.to_string()).or_default();
        stats.update(temperature);
        acc
    };

    let reduce_op = |mut acc: FxHashMap<String, Statistics>,
                     map: FxHashMap<String, Statistics>|
     -> FxHashMap<String, Statistics> {
        for (name, stats) in map {
            let acc_stats: &mut Statistics = acc.entry(name).or_default();
            acc_stats.merge(stats);
        }
        acc
    };

    let identity = FxHashMap::<String, Statistics>::default;

    let now: Instant = Instant::now();

    let mut result: FxHashMap<String, Statistics> = input
        .par_lines()
        .filter(filter_op)
        .map(map_op)
        .fold(identity, fold_op)
        .reduce(identity, reduce_op);
    result
        .values_mut()
        .for_each(|stats: &mut Statistics| stats.compute());

    let duration: Duration = now.elapsed();
    println!("Results for {} generated in {:?}", &cli.path, duration);

    let file: File = File::create(cli.path.replace("measurements", "results")).unwrap();
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

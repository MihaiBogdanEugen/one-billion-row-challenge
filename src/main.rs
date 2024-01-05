use clap::Parser;
use rayon::iter::ParallelIterator;
use rayon::prelude::*;
use rustc_hash::FxHashMap;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::PathBuf;
use std::time::Duration;
use std::time::Instant;

struct Measurement {
    name: String,
    temperature: i64,
}

impl From<&str> for Measurement {
    fn from(line: &str) -> Self {
        let (name_as_str, temperature_as_str) = line.split_once(';').unwrap();
        let name: String = name_as_str.to_owned();
        let temperature: i64 = (temperature_as_str.parse::<f64>().unwrap() * 10.0) as i64;
        Measurement { name, temperature }
    }
}

struct Statistics {
    name: String,
    min: i64,
    max: i64,
    sum: i64,
    count: usize,
}

impl Statistics {

    fn get_min(&self) -> f64 {
        format!("{:.1$}", (self.min as f64 / 10.0), 1)
            .parse::<f64>()
            .unwrap()        
    }

    fn get_max(&self) -> f64 {
        format!("{:.1$}", (self.max as f64 / 10.0), 1)
            .parse::<f64>()
            .unwrap()        
    }

    fn get_mean(&self) -> f64 {
        format!("{:.1$}", (self.sum as f64 / (10.0 * self.count as f64)), 1)
            .parse::<f64>()
            .unwrap()
    }
}

impl Default for Statistics {
    fn default() -> Self {
        Self {
            name: String::from(""),
            min: i64::MAX,
            max: i64::MIN,
            sum: 0,
            count: 0,
        }
    }
}

impl Display for Statistics {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}={}/{}/{}",
            self.name,
            self.get_min(),
            self.get_max(),
            self.get_mean()
        )
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

    let fold_op = |mut acc: FxHashMap<String, Statistics>, m: Measurement| -> FxHashMap<String, Statistics> {
        let stats: &mut Statistics = acc.entry(m.name.to_string()).or_default();
        stats.name = m.name;
        stats.min = stats.min.min(m.temperature);
        stats.max = stats.max.max(m.temperature);
        stats.sum += m.temperature;
        stats.count += 1;
        acc
    };

    let op = |mut acc: FxHashMap<String, Statistics>, map: FxHashMap<String, Statistics>| -> FxHashMap<String, Statistics> {
        for (name, stats) in map {
            let acc_stats: &mut Statistics = acc.entry(name).or_default();
            acc_stats.name = stats.name;
            acc_stats.min = acc_stats.min.min(stats.min);
            acc_stats.max = acc_stats.max.max(stats.max);
            acc_stats.sum += stats.sum;
            acc_stats.count += stats.count;
        }
        acc
    };

    let now: Instant = Instant::now();
    let result: FxHashMap<String, Statistics> = input
        .par_lines()
        .filter(|line: &&str| !line.is_empty())
        .map(|line: &str| Measurement::from(line))
        .fold(
            FxHashMap::<String, Statistics>::default,
            fold_op,
        )
        .reduce(FxHashMap::<String, Statistics>::default, op);
    let duration: Duration = now.elapsed();
    println!("Results for {} generated in {:?}", &cli.path, duration);

    let file: File = File::create(cli.path.replace("measurements", "results")).unwrap();
    let mut writer: BufWriter<File> = BufWriter::new(file);

    let mut sorted_result: Vec<&Statistics> =
        result.values().collect::<Vec<&Statistics>>();
    sorted_result.sort_by(
        |a: &&Statistics, b: &&Statistics| a.name.cmp(&b.name),
    );

    for statistics in sorted_result {
        writeln!(writer, "{}", statistics).unwrap();
    }
}

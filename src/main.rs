use clap::Parser;
use rayon::iter::ParallelIterator;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::PathBuf;
use std::time::Duration;
use std::time::Instant;

struct WeatherStationMeasurement {
    name: String,
    temperature: f64,
}

impl From<&str> for WeatherStationMeasurement {
    fn from(line: &str) -> Self {
        let (name_as_str, temperature_as_str) = line.split_once(';').unwrap();
        let name = name_as_str.to_owned();
        let temperature: f64 = temperature_as_str.parse::<f64>().unwrap();
        WeatherStationMeasurement { name, temperature }
    }
}

struct WeatherStationStatistics {
    name: String,
    min: f64,
    max: f64,
    sum: f64,
    count: usize,
}

impl WeatherStationStatistics {
    fn get_mean(&self) -> f64 {
        format!("{:.1$}", (self.sum / self.count as f64), 1)
            .parse::<f64>()
            .unwrap()
    }
}

impl Default for WeatherStationStatistics {
    fn default() -> Self {
        Self {
            name: String::from(""),
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
            sum: 0.0,
            count: 0,
        }
    }
}

impl Display for WeatherStationStatistics {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}={}/{}/{}",
            self.name,
            self.min,
            self.max,
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

    let now: Instant = Instant::now();
    let result: HashMap<String, WeatherStationStatistics> = input
        .par_lines()
        .filter(|line: &&str| !line.is_empty())
        .map(|line: &str| WeatherStationMeasurement::from(line))
        .fold(
            HashMap::<String, WeatherStationStatistics>::new,
            |mut acc: HashMap<String, WeatherStationStatistics>, m: WeatherStationMeasurement| {
                let stats: &mut WeatherStationStatistics =
                    acc.entry(m.name.to_string()).or_default();
                stats.name = m.name;
                stats.min = stats.min.min(m.temperature);
                stats.max = stats.max.max(m.temperature);
                stats.sum += m.temperature;
                stats.count += 1;
                acc
            },
        )
        .reduce(
            HashMap::<String, WeatherStationStatistics>::new,
            |mut acc: HashMap<String, WeatherStationStatistics>,
             map: HashMap<String, WeatherStationStatistics>| {
                for (name, stats) in map {
                    let acc_stats: &mut WeatherStationStatistics = acc.entry(name).or_default();
                    acc_stats.name = stats.name;
                    acc_stats.min = acc_stats.min.min(stats.min);
                    acc_stats.max = acc_stats.max.max(stats.max);
                    acc_stats.sum += stats.sum;
                    acc_stats.count += stats.count;
                }
                acc
            },
        );
    let duration: Duration = now.elapsed();
    println!("Results for {} generated in {:?}", &cli.path, duration);

    let file: File = File::create(cli.path.replace("measurements", "results")).unwrap();
    let mut writer: BufWriter<File> = BufWriter::new(file);

    let mut sorted_result: Vec<&WeatherStationStatistics> =
        result.values().collect::<Vec<&WeatherStationStatistics>>();
    sorted_result.sort_by(|a, b| a.name.cmp(&b.name));

    for statistics in sorted_result {
        writeln!(writer, "{}", statistics).unwrap();
    }
}

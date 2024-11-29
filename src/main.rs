use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::PathBuf;
use std::time::Instant;

use clap::Parser;
use clap::ValueEnum;
use obrc::solutions::solution_naive::SolutioNaive;
use obrc::solutions::solution_rayon_fxhash::SolutionRayonFxHash;
use obrc::solutions::solver::Solver;

#[derive(Parser)]
#[command()]
struct Cli {
    #[arg(short, long, default_value_t = String::from("measurements_1000000000.txt"))]
    input_path: String,

    #[arg(short, long, value_enum, default_value_t = Solution::Naive)]
    solution: Solution,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Solution {
    Naive,
    RayonFxHash,
}

fn main() {
    let cli: Cli = Cli::parse();

    assert!(
        PathBuf::from(&cli.input_path).exists(),
        "Path {:?} does not exist!",
        &cli.input_path
    );

    let now: Instant = Instant::now();
    let output: String = match cli.solution {
        Solution::Naive => SolutioNaive::solve_obrc(&cli.input_path),
        Solution::RayonFxHash => SolutionRayonFxHash::solve_obrc(&cli.input_path),
    };

    println!(
        "{:?} solution for {} generated in {:?}",
        cli.solution,
        &cli.input_path,
        now.elapsed()
    );

    let file: File = File::create(
        cli.input_path.replace(
            "measurements",
            format!("results_{:?}", cli.solution)
                .to_lowercase()
                .as_str(),
        ),
    )
    .unwrap_or_else(|_| panic!("Cannot create result file!"));
    let mut writer: BufWriter<File> = BufWriter::new(file);
    writeln!(writer, "{}", output).unwrap();
}

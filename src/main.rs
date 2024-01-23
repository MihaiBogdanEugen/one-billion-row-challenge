use obrc::solutions::rayon_map_fold_reduce::solve;
use obrc::solutions::rayon_map_fold_reduce::Statistics;
use obrc::utils::hash::FastHashMap;
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::env::args;
use std::fs::read_to_string;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::PathBuf;
use std::time::Instant;

fn main() {
    let path_str: String = args()
        .nth(1)
        .unwrap_or(String::from("measurements_1000000000.txt"));
    let path: PathBuf = PathBuf::from(&path_str);
    assert!(path.exists(), "Path {:?} does not exist!", path);

    let now: Instant = Instant::now();
    let input: String = read_to_string(path).unwrap();
    let result: FastHashMap<&str, Statistics> = solve(&input);
    println!(
        "[rayon_map_fold_reduce] Result for {} generated in {:?}",
        &path_str,
        now.elapsed()
    );

    let file: File = File::create(path_str.replace("measurements", "result"))
        .unwrap_or_else(|_| panic!("Cannot create result file!"));
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

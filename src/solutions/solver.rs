pub trait Solver {
    /// Solve the One Billion Row Challenge.
    /// Read from input_path file and write to output_path file.
    fn solve_obrc(input_path: &str) -> String;
}

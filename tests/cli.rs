#[cfg(test)]
mod tests {
    use std::fs;
    use assert_cmd::Command;

    type TestResult = Result<(), Box<dyn std::error::Error>>;
    const PRG: &str = "headr";
    const EMPTY: &str = "tests/inputs/empty.txt";
    const ONE_LINE: &str = "tests/inputs/one.txt";
    const TWO_LINES: &str = "tests/inputs/two.txt";
    const THREE_LINES: &str = "tests/inputs/three.txt";
    const TEN_LINES: &str = "tests/inputs/two.txt";
    const GLOB_FILES: &str = "tests/inputs/*.txt";
    #[test]
    #[should_panic]
    fn test_panic() {
        panic!()
    }

    #[test]
    #[ignore]
    fn reads_stdin() -> TestResult {
        let input = "hello\nworld\n";
        Command::cargo_bin(PRG)?
            .write_stdin(input)
            .assert()
            .success()
            .stdout(input);
        Ok(())
    }

    fn run(args: &[&str], expected_file: &str) -> TestResult {
        let expected = fs::read_to_string(expected_file)?;
        Command::cargo_bin(PRG)?
            .args(args)
            .assert()
            .success()
            .stdout(expected);
        Ok(())
    }

    #[test]
    #[ignore]
    fn test_empty() -> TestResult {
        run(&[EMPTY], EMPTY)
    }

    #[test]
    fn test_one() -> TestResult {
        run(&[ONE_LINE], ONE_LINE)
    }

    #[test]
    fn test_two() -> TestResult {
        run(&[TWO_LINES], TWO_LINES)
    }

    #[test]
    fn test_three() -> TestResult {
        run(&[THREE_LINES], THREE_LINES)
    }
    #[test]
    fn test_ten() -> TestResult {
        run(&[TEN_LINES], TEN_LINES)
    }

    #[test]
    fn test_file_glob_no_options() -> TestResult {
        run(&[GLOB_FILES], "tests/expected/glob_no_options.txt")
    }
    #[test]
    fn test_file_glob_char_option() -> TestResult {
        run(&[GLOB_FILES], "tests/expected/glob_six_chars.txt")
    }
    #[test]
    fn test_file_glob_line_option() -> TestResult {
        run(&[GLOB_FILES], "tests/expected/glob_six_lines.txt")
    }
}

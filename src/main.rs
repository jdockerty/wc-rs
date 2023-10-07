use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    /// Input file to execute actions on.
    input_file: Option<PathBuf>,

    #[arg(short, long, help = "count the number of bytes")]
    count: bool,

    #[arg(short, long, help = "count the number of lines")]
    lines: bool,

    #[arg(short, long, help = "count the number of words")]
    words: bool,

    #[arg(short = 'm', long, help = "count the number of characters")]
    chars: bool,
}

fn count_chars(file_path: PathBuf) -> Result<usize, std::io::Error> {
    let file = std::fs::read_to_string(file_path)?;

    // I think the usage of graphemes is actually "better", but chars().count()
    // comes out with the same output as the regular GNU wc utility, so for testability
    // and completeness, we'll go with that.
    // Ok(file.graphemes(true).count())
    Ok(file.chars().count())
}

fn count_bytes(file_path: PathBuf) -> Result<u64, std::io::Error> {
    let input_file = std::fs::File::open(file_path)?;
    let size_in_bytes = input_file.metadata()?.len();

    Ok(size_in_bytes)
}

fn count_lines(file_path: PathBuf) -> Result<usize, std::io::Error> {
    Ok(std::fs::read_to_string(file_path)?.lines().count())
}

fn count_words(file_path: PathBuf) -> Result<usize, std::io::Error> {
    let file = std::fs::read_to_string(file_path)?;
    let lines = file.lines();

    let mut wc = 0;
    for line in lines {
        wc += line.split_whitespace().count();
    }

    Ok(wc)
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    let Args {
        input_file,
        count,
        words,
        lines,
        chars,
    } = args;

    let input_file = match input_file {
        Some(file) => file,
        None => PathBuf::from("-"),
    };

    if count {
        let size_in_bytes = count_bytes(input_file.clone())?;
        println!("{size_in_bytes} {}", input_file.clone().to_string_lossy());
    }

    if lines {
        let lines = count_lines(input_file.clone())?;
        println!("{lines} {}", input_file.clone().to_string_lossy());
    }

    if words {
        let word_count = count_words(input_file.clone())?;
        println!("{word_count} {}", input_file.clone().to_string_lossy());
    }

    if chars {
        let char_count = count_chars(input_file.clone())?;
        println!("{char_count} {}", input_file.clone().to_string_lossy());
    }

    let line_count = count_lines(input_file.clone())?;
    let word_count = count_words(input_file.clone())?;
    let byte_count = count_bytes(input_file.clone())?;

    println!(
        "{line_count} {word_count} {byte_count} {}",
        input_file.clone().to_string_lossy()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_FILE: &str = "testdata/file.txt";

    #[test]
    fn count_bytes_returns_correctly() {
        let file_path = PathBuf::from(TEST_FILE);
        let expected_size: u64 = 342190;

        let file_size = count_bytes(file_path).expect("unable to count_bytes");

        assert_eq!(file_size, expected_size);
    }

    #[test]
    fn count_lines_returns_correctly() {
        let file_path = PathBuf::from(TEST_FILE);

        let expected: usize = 7145;

        let lines = count_lines(file_path).expect("unable to count lines in file");

        assert_eq!(expected, lines);
    }

    #[test]
    fn count_words_returns_correctly() {
        let file_path = PathBuf::from(TEST_FILE);

        let expected: usize = 58164;

        let words = count_words(file_path).expect("unable to count lines in file");

        assert_eq!(expected, words);
    }

    #[test]
    fn count_chars_returns_correctly() {
        let file_path = PathBuf::from(TEST_FILE);

        let expected: usize = 339292;

        let chars = count_chars(file_path).expect("unable to count lines in file");

        assert_eq!(expected, chars);
    }
}

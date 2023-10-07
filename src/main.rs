use clap::Parser;
use std::{
    io::{BufRead, BufReader, Read},
    path::PathBuf,
};

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

#[derive(Debug)]
/// Statistics for one or more input files.
struct Stats {
    bytes: usize,
    chars: usize,
    words: usize,
    lines: usize,
}

fn read_contents<R: Read>(reader: R) -> Result<Stats, std::io::Error> {
    let mut total_bytes = 0;
    let mut line_count = 0;
    let mut word_count = 0;
    let mut char_count = 0;

    let mut buf = BufReader::new(reader);
    let mut line = Vec::new();
    while buf.read_until(b'\n', &mut line)? > 0 {
        let parsed_line = String::from_utf8_lossy(&line);
        word_count += parsed_line.split_ascii_whitespace().count();
        char_count += parsed_line.chars().count();
        total_bytes += parsed_line.len();
        line_count += 1;
        line.clear();
    }

    Ok(Stats {
        bytes: total_bytes,
        words: word_count,
        lines: line_count,
        chars: char_count,
    })
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

    let input_path = &input_file.clone().unwrap_or(PathBuf::from("-"));
    let mut input: Box<dyn Read + 'static> = if input_path.as_os_str() == "-" {
        Box::new(std::io::stdin())
    } else {
        match std::fs::File::open(&input_path) {
            Ok(file) => Box::new(file),
            Err(err) => {
                return Err(err);
            }
        }
    };

    let file_stats = read_contents(&mut input)?;

    if count {
        println!("{} {}", file_stats.bytes, input_path.display());
        return Ok(());
    }

    if lines {
        println!("{} {}", file_stats.lines, input_path.display());
        return Ok(());
    }

    if words {
        println!("{} {}", file_stats.words, input_path.display());
        return Ok(());
    }

    if chars {
        println!("{} {}", file_stats.chars, input_path.display());
        return Ok(());
    }

    println!(
        "{} {} {} {}",
        file_stats.lines,
        file_stats.words,
        file_stats.bytes,
        input_path.display()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    const TEST_FILE: &str = "testdata/file.txt";

    // Implementation of PartialEq allows the use of assert_eq! macro
    // for a full test of the Stats struct.
    impl PartialEq for Stats {
        fn eq(&self, other: &Stats) -> bool {
            self.lines == other.lines
                && self.bytes == other.bytes
                && self.words == other.words
                && self.chars == other.chars
        }
    }

    #[test]
    fn count_bytes_returns_correctly() {
        let file = File::open(TEST_FILE).expect("unable to open test file");
        let expected_size: usize = 342190;

        let stats = read_contents(file).expect("unable to read contents");

        assert_eq!(stats.bytes, expected_size);
    }

    #[test]
    fn count_lines_returns_correctly() {
        let file = File::open(TEST_FILE).expect("unable to open test file");

        let expected: usize = 7145;

        let stats = read_contents(file).expect("unable to read contents");

        assert_eq!(expected, stats.lines);
    }

    #[test]
    fn count_words_returns_correctly() {
        let file = File::open(TEST_FILE).expect("unable to open test file");

        let expected: usize = 58164;

        let stats = read_contents(file).expect("unable to read contents");

        assert_eq!(expected, stats.words);
    }

    #[test]
    fn count_chars_returns_correctly() {
        let file = File::open(TEST_FILE).expect("unable to open test file");
        let expected: usize = 339292;

        let stats = read_contents(file).expect("unable to read contents");

        assert_eq!(expected, stats.chars);
    }

    #[test]
    fn count_read_contents_correctly() {
        let file = File::open(TEST_FILE).expect("unable to open test file");
        let expected = Stats {
            chars: 339292,
            words: 58164,
            lines: 7145,
            bytes: 342190,
        };

        let stats = read_contents(file).expect("unable to read contents");

        assert_eq!(expected, stats);
    }
}

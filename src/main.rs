use clap::Parser;
use std::{
    io::{BufRead, BufReader, Read},
    path::PathBuf,
};
use utf8_chars::BufReadCharsExt;

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

fn count_chars<R: Read>(input: R) -> Result<usize, std::io::Error> {
    let mut buf = BufReader::new(input);

    // I think the usage of graphemes is actually "better", but chars().count()
    // comes out with the same output as the regular GNU wc utility, so for testability
    // and completeness, we'll go with that.
    // Ok(file.graphemes(true).count())
    Ok(buf.chars().count())
}

fn count_bytes<R: Read>(input: R) -> Result<usize, std::io::Error> {
    let mut buf = BufReader::new(input);
    let mut total_bytes = 0;
    let mut buffer = Vec::new();

    while buf.read_until(b'\n', &mut buffer)? > 0 {
        total_bytes += buffer.len();
        buffer.clear();
    }
    Ok(total_bytes)
}

fn count_lines<R: Read>(input: R) -> Result<usize, std::io::Error> {
    let buf = BufReader::new(input);
    Ok(buf.lines().count())
}

fn count_words<R: Read>(input: R) -> Result<usize, std::io::Error> {
    let buf = BufReader::new(input);
    let lines = buf.lines();

    let mut wc = 0;
    for line in lines {
        wc += line?.split_whitespace().count();
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

    if count {
        let size_in_bytes = count_bytes(input)?;
        println!("{size_in_bytes} {}", input_path.display());
        return Ok(());
    }

    if lines {
        let lines = count_lines(input)?;
        println!("{lines} {}", input_path.display());
        return Ok(());
    }

    if words {
        let word_count = count_words(input)?;
        println!("{word_count} {}", input_path.display());
        return Ok(());
    }

    if chars {
        let char_count = count_chars(input)?;
        println!("{char_count} {}", input_path.display());
        return Ok(());
    }

    let line_count = count_lines(input.by_ref())?;
    let word_count = count_words(input.by_ref())?;
    let byte_count = count_bytes(input.by_ref())?;

    println!(
        "{line_count} {word_count} {byte_count} {}",
        input_path.display()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    const TEST_FILE: &str = "testdata/file.txt";

    #[test]
    fn count_bytes_returns_correctly() {
        let file = File::open(TEST_FILE).expect("unable to open test file");
        let expected_size: usize = 342190;

        let file_size = count_bytes(file).expect("unable to count_bytes");

        assert_eq!(file_size, expected_size);
    }

    #[test]
    fn count_lines_returns_correctly() {
        let file = File::open(TEST_FILE).expect("unable to open test file");

        let expected: usize = 7145;

        let lines = count_lines(file).expect("unable to count lines in file");

        assert_eq!(expected, lines);
    }

    #[test]
    fn count_words_returns_correctly() {
        let file = File::open(TEST_FILE).expect("unable to open test file");

        let expected: usize = 58164;

        let words = count_words(file).expect("unable to count lines in file");

        assert_eq!(expected, words);
    }

    #[test]
    fn count_chars_returns_correctly() {
        let file = File::open(TEST_FILE).expect("unable to open test file");

        let expected: usize = 339292;

        let chars = count_chars(file).expect("unable to count lines in file");

        assert_eq!(expected, chars);
    }
}

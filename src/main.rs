use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, help = "count the number of bytes in a file")]
    count: bool,

    #[arg(short, long)]
    file: PathBuf,

    #[arg(short, long, help = "count the number of lines in a file")]
    lines: bool,
}

fn count_bytes(file_path: PathBuf) -> Result<u64, std::io::Error> {
    let input_file = std::fs::File::open(&file_path)?;
    let size_in_bytes = input_file.metadata()?.len();

    Ok(size_in_bytes)
}

fn count_lines(file_path: PathBuf) -> Result<usize, std::io::Error> {
    Ok(std::fs::read_to_string(file_path)?.lines().count())
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    match args {
        Args { count, file, lines } => {
            if count {
                let size_in_bytes = count_bytes(file.to_path_buf())?;
                println!("{size_in_bytes} {}", file.to_string_lossy());
            }

            if lines {
                let lines = count_lines(file.to_path_buf())?;
                println!("{lines} {}", file.to_string_lossy());
            }

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_bytes_returns_correctly() {
        let file_path = PathBuf::from("testdata/file.txt");
        let expected_size: u64 = 342190;

        let file_size = count_bytes(file_path).expect("unable to count_bytes");

        assert_eq!(file_size, expected_size);
    }

    #[test]
    fn count_lines_returns_correctly() {
        let file_path = PathBuf::from("testdata/file.txt");

        let expected: usize = 7145;

        let lines = count_lines(file_path).expect("unable to count lines in file");

        assert_eq!(expected, lines);

    #[test]
    fn count_bytes_returns_correctly() {
        let file = NamedTempFile::new().expect("unable to create temporary file");
        let execpted_size: u64 = 5000;
        file.as_file()
            .set_len(execpted_size)
            .expect("unable to set file size");

        let file_size = count_bytes(file.path().into()).expect("unable to count_bytes");

        assert_eq!(file_size, execpted_size);
    }
}

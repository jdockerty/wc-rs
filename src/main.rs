use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, help = "count the number of bytes in a file")]
    count: bool,

    #[arg(short, long)]
    file: PathBuf,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    match args {
        Args { count, file } => {
            if count {
                let input_file = std::fs::File::open(&file)?;
                let size_in_bytes = input_file.metadata()?.len();
                println!("{size_in_bytes} {}", file.to_string_lossy());
            }

            Ok(())
        }
    }
}

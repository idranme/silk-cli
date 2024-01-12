use bytes::Buf;
use clap::{Parser, ValueEnum};
use silk_rs::{decode_silk, encode_silk};
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help = true, bin_name = "silk-cli")]
struct Cli {
    /// Path of the input file
    #[arg(short = 'i', long = "input")]
    input: String,

    /// Path of the output file
    #[arg(short = 'o', long = "output")]
    output: String,

    /// Sample rate of the input file
    #[arg(short = 's', long = "sample-rate")]
    sample_rate: i32,

    /// Operating mode
    #[arg(short = 'm', long = "mode", value_enum, default_value_t = Mode::Encode)]
    mode: Mode,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    Encode,
    Decode,
}

fn main() {
    let cli = Cli::parse();

    let input = std::fs::read(Path::new(&cli.input)).unwrap();
    match cli.mode {
        Mode::Encode => {
            let output = encode_silk(input, cli.sample_rate, 25000, true).unwrap();
            std::fs::write(Path::new(&cli.output), &output[0..output.len() - 1]).unwrap();

            let mut offset = 10;
            let mut i = 0;
            while offset < output.len() {
                let mut buf = &output[offset..];
                let size = buf.get_u16_le();
                offset += 2;
                i += 1;
                offset += size as usize;
            }
            print!("Duration: {}\n", i * 20)
        }
        Mode::Decode => {
            let output = decode_silk(input, cli.sample_rate).unwrap();
            std::fs::write(Path::new(&cli.output), output).unwrap();
        }
    }

    println!("Done!")
}

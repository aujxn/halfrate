use hound::WavReader;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    // The wav file to resample
    #[structopt(
        short,
        long,
        default_value = "hw-resample/gc.wav hw-resample/sine.wav hw-resample/synth.wav"
    )]
    files: String,

    // The target sample rate as a fraction of the old rate.
    // Default value is 0.5.
    #[structopt(short, long, default_value = "0.5")]
    rate: f64,
}

fn main() {
    let opt = Opt::from_args();
    let files: Vec<String> = opt
        .files
        .as_str()
        .split_ascii_whitespace()
        .map(|x| x.to_string())
        .collect();

    for file in files {
        let wav = WavReader::open(&file).unwrap();

        println!("File: {}\n{:?}", file, wav.spec());
    }
}

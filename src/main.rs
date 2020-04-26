use hound::WavReader;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    // The wav file to resample
    #[structopt(short, long)]
    file: String,

    // The target sample rate as a fraction of the old rate.
    // Default value is 0.5.
    #[structopt(short, long, default_value = "0.5")]
    rate: f64,
}

fn main() {
    let opt = Opt::from_args();
    let wav = WavReader::open(opt.file).unwrap();

    println!("{:?}", wav.spec());
}

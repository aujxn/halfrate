use hound::{SampleFormat, WavReader, WavSamples, WavSpec, WavWriter};
use sdr::fir::FIR;
use std::fs::File;
use std::io::BufWriter;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    // The wav file to resample
    #[structopt(short, long, default_value = "gc.wav sine.wav synth.wav")]
    files: String,

    // The target sample rate as a fraction of the old rate.
    // Default value is 0.5.
    // Must be 0.0 < x < 1.0
    #[structopt(short, long, default_value = "0.5")]
    rate: f64,
}

fn main() {
    let opt = Opt::from_args();

    if opt.rate <= 0.0 || opt.rate >= 1.0 {
        println! {"rate must be 0.0 < x < 1.0"};
        return;
    }

    let files: Vec<String> = opt
        .files
        .as_str()
        .split_ascii_whitespace()
        .map(|x| x.to_string())
        .collect();

    for in_file in files {
        let mut wav = WavReader::open(&in_file).unwrap();
        let spec = wav.spec();
        let target_rate = spec.sample_rate as f64 * opt.rate;

        println!(
            "File: {}\n{:?}\nTarget Rate: {}\n",
            in_file, spec, target_rate
        );

        let samples: WavSamples<_, i16> = wav.samples();
        let mut filter: FIR<i16> = FIR::lowpass(50, opt.rate * 0.5);

        let filtered = filter.process(&samples.map(|x| x.unwrap()).collect::<Vec<i16>>());
        let decimated = filtered.into_iter().step_by(2).collect::<Vec<i16>>();

        let out_file = File::create("r".to_string() + &in_file).unwrap();
        let writer = BufWriter::new(out_file);

        let spec = WavSpec {
            channels: 1,
            sample_rate: target_rate as u32,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int,
        };

        let mut writer = WavWriter::new(writer, spec).unwrap();
        for sample in decimated {
            writer.write_sample(sample);
        }
        writer.finalize();
    }
}

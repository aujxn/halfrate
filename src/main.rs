use hound::{SampleFormat, WavReader, WavSpec, WavWriter};
use rayon::prelude::*;
use std::fs::File;
use std::io::BufWriter;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    // The wav file to resample
    #[structopt(short, long, default_value = "gc.wav sine.wav synth.wav")]
    files: String,
}

fn main() {
    let opt = Opt::from_args();

    let files: Vec<String> = opt
        .files
        .as_str()
        .split_ascii_whitespace()
        .map(|x| x.to_string())
        .collect();

    for in_file in files {
        let mut wav = WavReader::open(&in_file).unwrap();
        let spec = wav.spec();

        println!("File: {}\n{:?}\n", in_file, spec);

        let samples = wav
            .samples()
            .map(|x: Result<i16, _>| x.unwrap() as f64)
            .collect();
        let decimated = convolve_and_decimate(samples);
        write_wav(decimated, in_file);
    }
}

fn write_wav(samples: Vec<i16>, in_file: String) {
    let out_file = File::create("r".to_string() + &in_file).unwrap();
    let writer = BufWriter::new(out_file);

    let spec = WavSpec {
        channels: 1,
        sample_rate: 24000,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    let mut writer = WavWriter::new(writer, spec).unwrap();
    for sample in samples {
        writer.write_sample(sample).unwrap();
    }
    writer.finalize().unwrap();
}

fn convolve_and_decimate(input: Vec<f64>) -> Vec<i16> {
    // low half pass FIR coefficients
    let coef: [f64; 91] = [
        0.0007402732643054117,
        -0.0007116178196449282,
        -0.0012364542225753877,
        0.0004878382037768346,
        0.001762355408397323,
        -4.42067370785638e-18,
        -0.002215182400043849,
        -0.0007717219779807554,
        0.002468118275890691,
        0.0017997462427444813,
        -0.002385864864076639,
        -0.0029997753194695915,
        0.0018453409331850108,
        0.004228422134073137,
        -0.0007592690890261643,
        -0.005289349745070121,
        -0.0009003571537489589,
        0.005948391131440519,
        0.0030822569848409777,
        -0.005956801424111659,
        -0.00564241768842927,
        0.005080467069416566,
        0.008339545358107981,
        -0.003131762701341885,
        -0.01084034550601907,
        1.3106206228765793e-17,
        0.012733594561465547,
        0.004323800421297494,
        -0.013549309312476557,
        -0.009731690323690397,
        0.012775497060241557,
        0.015996022350404576,
        -0.009858263926207278,
        -0.022780555925105705,
        0.004155841182759176,
        0.029665112243643956,
        0.005226581766628221,
        -0.036181637361030625,
        -0.019949557006721064,
        0.04185792449918842,
        0.044243745670944155,
        -0.04626408217804201,
        -0.09396783582341778,
        0.04905629457673092,
        0.3142499866912212,
        0.45011339147505103,
        0.3142499866912212,
        0.04905629457673092,
        -0.09396783582341778,
        -0.04626408217804201,
        0.044243745670944155,
        0.04185792449918842,
        -0.019949557006721064,
        -0.036181637361030625,
        0.005226581766628221,
        0.029665112243643956,
        0.004155841182759176,
        -0.022780555925105705,
        -0.009858263926207278,
        0.015996022350404576,
        0.012775497060241557,
        -0.009731690323690397,
        -0.013549309312476557,
        0.004323800421297494,
        0.012733594561465547,
        1.3106206228765793e-17,
        -0.01084034550601907,
        -0.003131762701341885,
        0.008339545358107981,
        0.005080467069416566,
        -0.00564241768842927,
        -0.005956801424111659,
        0.0030822569848409777,
        0.005948391131440519,
        -0.0009003571537489589,
        -0.005289349745070121,
        -0.0007592690890261643,
        0.004228422134073137,
        0.0018453409331850108,
        -0.0029997753194695915,
        -0.002385864864076639,
        0.0017997462427444813,
        0.002468118275890691,
        -0.0007717219779807554,
        -0.002215182400043849,
        -4.42067370785638e-18,
        0.001762355408397323,
        0.0004878382037768346,
        -0.0012364542225753877,
        -0.0007116178196449282,
        0.0007402732643054117,
    ];

    // vector mult
    let dot = |x: &[f64], y: &[f64]| x.iter().zip(y.iter()).fold(0.0, |acc, (x, y)| acc + x * y);

    // convert f64 to i16 without overflow
    let f64_to_i16_safe = |x: f64| -> i16 {
        let max = std::i16::MAX as f64;
        if x > max {
            std::i16::MAX
        } else if x < -1.0 * max {
            -1 * std::i16::MAX
        } else {
            x as i16
        }
    };

    // convolve coefficients over input and skip every other sample to half the rate
    (0..input.len())
        .into_par_iter()
        // skip evey other sample to decimate input to halfrate
        .filter(|i| i % 2 == 1)
        .map(|i| {
            // get the first 45 samples unchanged
            if i < 91 {
                input[i] as i16
            // otherwise do the convolution
            } else {
                let product = dot(&coef, &input[i - 91..i]);
                f64_to_i16_safe(product)
            }
        })
        .collect()
}

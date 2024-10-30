use std::path::Path;

pub fn export_wav(
    channels: u16,
    sample_rate: u32,
    bits_per_sample: u16,
    samples: Vec<i16>,
    path: &Path,
) {
    let spec = hound::WavSpec {
        channels,
        sample_rate,
        bits_per_sample,
        sample_format: hound::SampleFormat::Int,
    };
    let path = Path::new(path);
    let mut writer = hound::WavWriter::create(path, spec).unwrap();
    for sample in samples {
        writer.write_sample(sample).unwrap();
    }
}

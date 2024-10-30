use std::{path::Path, vec};

use domain::{
    pure_tone::{PureTones, ToneAndDuration},
    text2track::Text2Track,
    text_analyzer::TextAnalyzer,
};

mod domain;
mod infrastructure;
use infrastructure::export_wav::export_wav;

fn main() {
    let sample_rate = 44100;
    let input = "Why Japanese people!?".into();
    let text_analyzer = TextAnalyzer::new(input);
    let text2track = Text2Track::new(text_analyzer);
    let track = text2track.generate_track();
    let s = PureTones::new(sample_rate, track);

    export_wav(1, sample_rate, 16, s.samples, Path::new("sine.wav"));
}

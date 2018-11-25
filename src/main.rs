//! Records a WAV file (roughly 3 seconds long) using the default input device and format.
//!
//! The input data is recorded to "$CARGO_MANIFEST_DIR/recorded.wav".

extern crate cpal;
extern crate hound;

mod recorder;

fn main() {
    recorder::record();
}

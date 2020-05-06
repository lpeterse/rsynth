use libpulse_binding as pulse;
use libpulse_simple_binding as psimple;
use psimple::Simple;
use pulse::sample;
use pulse::stream::Direction;
use std::error::Error;

use crate::*;

pub fn play<S: Signal<Sample = Voltage>>(
    signal: &mut S,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let spec = sample::Spec {
        format: sample::Format::S16le,
        channels: 1,
        rate: 44100,
    };

    //let mut x = sine(hz(440.0));
    //println!("{:?}", x.sample());

    let s: Simple = Simple::new(
        None,                   // Use the default server
        env!("CARGO_PKG_NAME"), // Our application’s name
        Direction::Playback,    // We want a playback stream
        None,                   // Use the default device
        "",                     // Description of our stream
        &spec,                  // Our sample format
        None,                   // Use default channel map
        None,                   // Use default buffering attributes
    )
    .unwrap();

    let mut buf: [u8; 2048] = [0; 2048];

    while s.write(&buf).is_ok() {
        for i in 0..1024 {
            let voltage = signal.sample();
            let voltage = (voltage.0 * 2f32.powi(15)) as i16;
            let [a, b] = voltage.to_le_bytes();
            unsafe {
                *buf.get_unchecked_mut(i * 2) = a;
                *buf.get_unchecked_mut(i * 2 + 1) = b;
            }
        }
    }
    Ok(())
}

use std::error::Error;

use rsynth::*;

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let clock0 = clock(hz(4.0));
    let seq0 = Sequence::new(clock0, vec![C3, D3, E3, F3, G3, A3, H3, C4]);
    let seq0 = multiple(seq0);

    let mut s1 = sine(seq0.clone());
    let mut s2 = square(seq0.clone());
    let mut s3 = saw(seq0);

    let mut z1 = sum!(s1, s2, s3);
    //let mut x = sine(hz(220.0));
    play(&mut z1)
    //Ok(())
}

use std::error::Error;

use rsynth::*;

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let clock0 = clock(hz(4.0));
    let seq0 = Sequence::new(clock0, vec![C3, D3, E3, F3, G3, A3, H3, C4]);
    let seq0 = multiple(seq0);

    let x = sine(hz(0.25));

    //let mut s1 = sine(seq0.clone());
    //let mut s2 = square(seq0.clone());
    let mut sin1 = sine(hz(110.0));
    let mut sin2 = sine(hz(111.0));
    let mut sin3 = sine(hz(112.0));
    let mut sqr0 = square(hz(109.5));
    let mut sin = sum!(sin1, sin2, sin3, Amp::new(Amp::new(sqr0, 0.5), x));
    //println!("{}", (2.0f32 * (2.0f32.powi(31))));

    //let mut z1 = sum!(s1, s2, s3);
    //let mut x = sine(hz(220.0));
    play(&mut sin);
    //play(&mut z1)
    Ok(())
}

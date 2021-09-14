use enigo::*;
use rand::prelude::*;
use rand::distributions::Standard;
use std::{thread, time};

fn hold(enigo: &mut Enigo, key: char, mut duration: f32, natural: bool) {
    let v: f32 = 0.3 * if natural { StdRng::from_entropy().sample(Standard) } else { 0.0 };
    duration += v;
    let t = time::Instant::now();
    while (time::Instant::now() - t).as_secs_f32() < duration {
        enigo.key_down(Key::Layout(key));
    }
    enigo.key_up(Key::Layout(key));
}

fn main() {
    let mut enigo = Enigo::new();
    println!("waiting...");
    thread::sleep(time::Duration::new(5, 0));
    println!("go");
    thread::sleep(time::Duration::new(1, 0));
    for _ in 0..50 {
        hold(&mut enigo, 'd', 0.3, true);
        hold(&mut enigo, 'a', 0.3, true);
    }
    
}

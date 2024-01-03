use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        k: usize,
        g: usize,
        m: usize,
    };

    let mut glass = 0;
    let mut mag = 0;

    for _ in 0..k {
        if glass == g {
            glass = 0;
        } else if mag == 0 {
            mag = m;
        } else {
            let diff = min(mag, g - glass);
            glass += diff;
            mag -= diff;
        }
    }

    println!("{} {}", glass, mag);
}

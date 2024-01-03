#[allow(unused_imports)]
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        mm: usize,
        dd: usize,
        y: usize,
        m: usize,
        d: usize,
    };

    if mm == m && dd == d {
        println!("{} {} {}", y + 1, 1, 1)
    } else if dd == d {
        println!("{} {} {}", y, m + 1, 1)
    } else {
        println!("{} {} {}", y, m, d + 1)
    }
}

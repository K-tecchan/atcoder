#[allow(unused_imports)]
use proconio::input;
use std::cmp::*;

fn main() {
    input! {n: usize};
    for x in 0..=n {
        for y in 0..=n {
            for z in 0..=n {
                if x + y + z <= n {
                    println!("{} {} {}", x, y, z);
                } else {
                    break;
                }
            }
        }
    }
}

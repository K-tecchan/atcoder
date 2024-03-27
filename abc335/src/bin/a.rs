#[allow(unused_imports)]
use proconio::input;
use std::cmp::*;

fn main() {
    input! {s: String};

    let ans = &s[..s.len() - 1];
    println!("{}4", ans);
}

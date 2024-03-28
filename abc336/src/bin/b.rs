#[allow(unused_imports)]
use proconio::input;
use std::cmp::*;

fn main() {
    input! {mut n: usize};
    let mut ans = 0;
    while n % 2 == 0 {
        n /= 2;
        ans += 1;
    }
    println!("{}", ans);
}

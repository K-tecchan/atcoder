#[allow(unused_imports)]
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n]
    };

    let mut ans = 0;
    for i in a {
        if i >= m {
            ans += 1;
        };
    }

    println!("{}", ans);
}

#[allow(unused_imports)]
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        x: [(usize, usize); n],
    };

    let mut x_ans = 0;
    let mut y_ans = 0;

    for (i, j) in x {
        x_ans += i;
        y_ans += j;
    }

    if x_ans > y_ans {
        println!("Takahashi");
    } else if x_ans < y_ans {
        println!("Aoki");
    } else {
        println!("Draw");
    }
}

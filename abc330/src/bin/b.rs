#[allow(unused_imports)]
use proconio::input;
use std::cmp::*;

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        a: [usize; n]
    };

    let mut ans = Vec::new();
    for i in a {
        if i <= l {
            ans.push(l.to_string());
        } else if i >= r {
            ans.push(r.to_string());
        } else {
            ans.push(i.to_string());
        }
    }

    println!("{}", ans.join(" "));
}

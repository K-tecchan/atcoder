use proconio::input;

fn main() {
    input! {
        a: i128,
        m: i128,
        l: i128,
        r: i128
    };

    let l = l - a;
    let r = r - a;

    println!("{}", r.div_euclid(m) - (l - 1).div_euclid(m));
}

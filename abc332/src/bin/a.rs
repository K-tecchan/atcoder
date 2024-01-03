use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        k: usize,
    };

    let mut ans = 0;
    for i in 0..n {
        input! {
            p: usize,
            q: usize,
        };

        ans += p * q;
    }

    if ans < s {
        println!("{}", ans + k);
    } else {
        println!("{}", ans);
    }
}

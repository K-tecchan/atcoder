use proconio::input;

fn main() {
    input! {
        N: usize,
        P: [usize; N],
    }
    
    let mut ans = 0;
    let mut min = P[0];
    for n in P.iter() {
        if n <= &min {
            ans += 1;
            min = *n;
        }
    }

    println!("{}", ans);
}

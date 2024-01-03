use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }
    
    let mut ans = 0;
    let mut min = p[0];
    for n in p.iter() {
        if n <= &min {
            ans += 1;
            min = *n;
        }
    }

    println!("{}", ans);
}

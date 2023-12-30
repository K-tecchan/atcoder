use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }
    let a_ans = a.to_string().repeat(b);
    let b_ans = b.to_string().repeat(a);
    let mut ans = vec![a_ans, b_ans];
    ans.sort();

    println!("{}", ans[0]);
}

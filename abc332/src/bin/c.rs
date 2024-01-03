use proconio::input;

fn main() {
    input! {
        _n: usize,
        m: usize,
        s: String
    };

    let mut shirt = m;
    let mut logo = 0;
    let mut ans = 0;

    for i in s.chars() {
       if i == '0' {
            shirt = m;
            logo = ans;
       } else if i == '1' {
            if shirt > 0 {
                shirt -= 1;
            } else if logo > 0{
                logo -= 1;
            } else {
                ans += 1;
            }
       } else {
            if logo > 0 {
                logo -= 1;
            } else {
                ans += 1;
            }
       }
    };

    println!("{}", ans);
}

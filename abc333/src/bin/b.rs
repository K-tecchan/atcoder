use proconio::input;

fn main() {
    input! {
        s: String,
        t: String
    };

    let pentagon = ['a', 'b', 'c', 'd', 'e'];
    let abs_s = pentagon.iter().position(|&c| c == s.chars().nth(0).unwrap()).unwrap();
    println!("{}", abs_s);
}

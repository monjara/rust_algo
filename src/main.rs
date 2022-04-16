use proconio::input;

fn main() {
    input! {s: String}
    let ans = '0'.to_string() + &s[0..=2];
    println!("{}", ans);
}

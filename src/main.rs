use proconio::input;

fn main() {
    input! {
        _: u32,
        s: String,
    };
    let ans = s.chars().last().unwrap();
    println!("{}", ans);
}

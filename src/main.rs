use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let ans = if a <= b && b <= a * 6 { "Yes" } else { "No" };
    println!("{}", ans);
}

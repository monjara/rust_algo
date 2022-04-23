use proconio::input;

fn main() {
    input! {
        h: f64,
    };
    let ans = f64::sqrt(h * (12800000 as f64 + h));
    println!("{}", ans);
}

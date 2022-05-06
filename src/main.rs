use proconio::input;

fn main() {
    input! {
        v: usize,
        t: usize,
        s: usize,
        d: usize,
    }
    let ans = if d < v * t || v * s < d {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}

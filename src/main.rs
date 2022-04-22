use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        c: f64,
        x: f64,
    };
    let ans = if x <= a {
        1.00
    } else if x <= b {
        c / (b - a)
    } else {
        0.00
    };
    println!("{:.13}", ans);
}

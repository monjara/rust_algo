use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        x: usize,
        y: usize,
    };
    let ans = if n > a {
        a * x + (n - a) * y
    } else {
        n * x
    };
    println!("{}", ans);
}

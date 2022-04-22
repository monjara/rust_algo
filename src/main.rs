use proconio::input;

fn main() {
    input! {
        mut a: f64,
        b: f64,
        k: f64,
    }
    let mut ans = 0;
    while b > a {
        a *= k;
        ans += 1;
    }
    println!("{}", ans);
}

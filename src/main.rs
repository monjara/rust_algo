use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut ans = 0;
    for _i in 0..n {
        input! {
            a: usize,
        };
        ans += if a > 10 { a - 10 } else { 0 };
    }
    println!("{}", ans);
}

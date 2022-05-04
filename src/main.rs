use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
    }
    let mut ans = 0;
    for _i in 0..n {
        input! {
            a: usize,
        }
        if a < p {
            ans += 1;
        }
    }
    println!("{}", ans);
}

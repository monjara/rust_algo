use proconio::input;

fn main() {
    input! {
        x: i64,
    };
    let ans = x.div_euclid(10);
    println!("{}", ans);
}

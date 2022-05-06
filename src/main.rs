use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    }
    let ans = 32usize.pow(a - b);
    println!("{}", ans);
}

use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let ans = if n <= 125 {
        4
    } else if n <= 211 {
        6
    } else {
        8
    };
    println!("{}", ans);
}

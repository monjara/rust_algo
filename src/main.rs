use proconio::input;

fn main() {
    input! {
       x: usize,
       y: usize,
    };
    let ans = if x == y {x} else {3-(x + y)};
    println!("{}", ans);
}

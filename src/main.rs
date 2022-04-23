use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
    };
    let mut total = 0;
    for i in 0..n {
        input!{
            mut a: usize,
        }
        if i % 2 == 1 { a -= 1; };
        total += a;
    }
    let ans = if x >= total { "Yes" } else { "No" };
    println!("{}", ans);
}

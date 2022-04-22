use proconio::input;

fn main() {
    input! {
        v: usize,
        a: usize,
        b: usize,
        c: usize,
    };
    let rest = v % (a + b + c);
    let ans = if rest < a {
        'F'
    } else if rest < a + b {
        'M'
    } else {
        'T'
    };
    println!("{}", ans);
}

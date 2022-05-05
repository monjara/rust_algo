use proconio::input;

fn main() {
    input! {
        mut n: usize,
        s: String,
    }
    n -= 1;
    let ans = if s.chars().nth(n).unwrap() == 'o' {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans)
}

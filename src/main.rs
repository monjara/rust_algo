use proconio::input;

fn main() {
    input! {
        x: usize,
    }
    let mut ans = 0;
    if x < 40 {
        ans = 40 - x;
    } else if x < 70 {
        ans = 70 - x;
    } else if x < 90 {
        ans = 90 - x;
    } else {
        println!("expert");
        return;
    }
    println!("{}", ans);
}

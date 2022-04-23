use proconio::input;

fn main() {
    input! {
        n: f64,
    };
    let price = (n * 1.08).floor() as usize;
    let ans = if price < 206 {
        "Yay!"
    } else if price == 206 {
        "so-so"
    } else {
        ":("
    };
    println!("{}", ans);
}

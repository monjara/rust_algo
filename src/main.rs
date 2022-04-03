use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        d: u32,
    }
    if a < c { println!("Takahashi") }
    else if a == c && b <= d { println!("Takahashi") }
    else { println!("Aoki") }
}

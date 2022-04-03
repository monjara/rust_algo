use proconio::input;

fn main() {
    input! {
        x1: i32, y1: i32,
        x2: i32, y2: i32,
        x3: i32, y3: i32,
    }
    let (x, y) = (x1^x2^x3, y1^y2^y3);
    println!("{} {}", x, y)
}

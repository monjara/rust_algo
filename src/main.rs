use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
    }
    for _i in 0..n {
        input! {
            a: usize,
        }
        if a != x {
            print!("{} ", a);
        }
    }
    println!();
}

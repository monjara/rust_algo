use proconio::input;

fn s(n: usize) {
    if n == 1 {
        print!("{} ", n);
        return;
    } else {
        s(n - 1);
        print!("{} ", n);
        s(n - 1);
    }
}

fn main() {
    input! {
        n: usize,
    }
    s(n);
}

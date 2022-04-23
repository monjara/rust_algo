use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    };
    a.sort();
    for i in 0..n {
        if i + 1 != a[i] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

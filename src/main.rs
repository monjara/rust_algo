use proconio::input;

fn main() {
    input! {
        mut n: [usize; 3],
    };
    n.sort();
    let ans = n[1] + n[2];
    println!("{}", ans);
}

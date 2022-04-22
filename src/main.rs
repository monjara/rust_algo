use proconio::input;

fn main() {
    input! {
        mut v: [i32; 10],
    }
    v.sort_by(|a, b| b.cmp(a));
    for i in 0..3 {
        println!("{}", v[i]);
    }
}

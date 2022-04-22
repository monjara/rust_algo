use proconio::input;

fn main() {
    input! {s: String}
    for i in 0..10 {
        if !s.contains(&i.to_string()) {
            println!("{}", i);
            return;
        }
    }
}

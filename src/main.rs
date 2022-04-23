use std::usize;

use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut money = 0;
    let mut i = 0;
    while money < n {
        i += 1;
        money += i;
    }
    println!("{}", i);
}

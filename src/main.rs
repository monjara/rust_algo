use std::cmp;
use std::io;

fn read() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s
}

fn main() {
    let n: isize = read().trim_end().parse().unwrap();
    let mut min: isize = read().trim_end().parse().unwrap();
    let mut diff = -1000000005;
    for _i in 0..n - 1 {
        let r = read().trim_end().parse().unwrap();
        let tmp = r - min;
        diff = cmp::max(diff, tmp);
        min = cmp::min(min, r);
    }
    println!("{}", diff);
}

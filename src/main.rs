use proconio::input;

fn main() {
    input! {
        _n: u64,
        t: String,
    };
    let mut x = 0;
    let mut y = 0;
    let mut d = 0;
    for c in t.chars() {
        if c == 'R' {
            d += 1;
            d %= 4;
        } else {
            if d == 0 {
                x += 1;
            } else if d == 1 {
                y -= 1;
            } else if d == 2 {
                x -= 1;
            } else if d == 3 {
                y += 1;
            }
        }
    }
    println!("{} {}", x, y);
}

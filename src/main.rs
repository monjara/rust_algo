use proconio::input;

fn check(s: &str) -> bool {
    let mut check = 0;
    for c in s.chars() {
        if c == '(' {
            check += 1
        } else if c == ')' {
            check -= 1
        }
        if check < 0 {
            return false;
        };
    }
    if check == 0 {
        true
    } else {
        false
    }
}

fn main() {
    input! {
        n: usize,
    }

    for bit in 0..(1 << n) {
        let mut str = "".to_string();
        for i in (0..=n - 1).rev() {
            if (bit & (1 << i)) == 0 {
                str.push_str("(");
            } else {
                str.push_str(")");
            }
        }
        if check(&str) {
            println!("{}", str);
        };
    }
}

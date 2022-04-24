use proconio::input;

fn solve(m: isize, k: &isize, l: &isize, v: &Vec<isize>) -> bool {
    let mut cnt = 0;
    let mut pre = 0;

    for i in v {
        if i - pre >= m && l - i >= m {
            cnt += 1;
            pre = *i;
        };
    }
    if &cnt >= k {
        true
    } else {
        false
    }
}

fn main() {
    input! {
        n: isize,
        l: isize,
        k: isize,
        a: [isize; n],
    }

    let mut left = -1;
    let mut right = l + 1;

    while right - left > 1 {
        let mid = left + (right - left) / 2;
        if solve(mid, &k, &l, &a) {
            left = mid;
        } else {
            right = mid;
        }
    }
    println!("{}", left);
}

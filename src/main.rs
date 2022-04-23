use proconio::input;

fn binary_search(key: usize, v: &Vec<usize>) -> isize {
    let mut left = 0;
    let mut right = v.len() - 1;
    while right - left > 1 {
        let mid = left + (right - left) / 2;
        if v[mid] > key {
            right = mid;
        } else {
            left = mid;
        }
    }
    right as isize
}

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let index = if k > a[n-1] {
        -1
    } else {
        binary_search(k, &a)
    };
    println!("{}", index);
}

// use proconio::input;

fn is_ok(right: usize, key: usize, v: &Vec<usize>) -> bool {
    v[right] >= key
}

fn binary_search(key: usize, v: &Vec<usize>) -> isize {
    let mut left = -1;
    let mut right = v.len() as isize;
    while right - left > 1 {
        let mid = left + (right - left) / 2;
        if is_ok(mid as usize, key, v) {
            right = mid;
        } else {
            left = mid;
        }
    }
    right
}

fn main() {
    let v = vec![1, 14, 32, 51, 51, 51, 243, 419, 750, 910];
    let index = binary_search(51, &v);
    println!("{}", index);
}

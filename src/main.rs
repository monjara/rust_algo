// use proconio::input;

fn binary_search(key: usize, v: &Vec<usize>) -> isize {
    let mut left = 0;
    let mut right = v.len() - 1;
    while right >= left {
        let mid = left + (right - left) / 2;
        if v[mid] == key {
            return mid as isize;
        } else if v[mid] > key {
            right = mid - 1;
        } else if v[mid] < key {
            left = mid + 1;
        }

    }
    return -1;
}

fn main() {
    let v = vec![1, 14, 32, 51, 51, 51, 243, 419, 750, 910];
    let index = binary_search(51, &v);
    println!("{}", index);
}

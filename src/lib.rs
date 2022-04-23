fn binary_search(key: usize, v: &Vec<usize>) -> isize {
    let mut left = -1;
    let mut right = (v.len()-1) as isize;
    while right - left > 1 {
        let mid = left + (right - left) / 2;
        if v[mid as usize] >= key {
            right = mid;
        } else {
            left = mid;
        }
    }
    right
}

#[test]
fn test_binary_search() {
    let v = vec![1, 14, 32, 51, 51, 51, 243, 419, 750, 910];
    assert_eq!(binary_search(1, &v), 0);
    assert_eq!(binary_search(51, &v), 3);
    assert_eq!(binary_search(910, &v), 9);
    assert_eq!(binary_search(1000, &v), 9);
}

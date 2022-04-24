fn read_a_word<T: std::str::FromStr>() -> Result<T, T::Err> {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s)
        .expect("could not read from stdin");
    s.trim_end().parse()
}

fn read_vector<T: std::str::FromStr>() -> Result<Vec<T>, T::Err> {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s)
        .expect("could not read from stdin");
    s.trim()
        .split_whitespace()
        .map(|word| word.parse())
        .collect()
}

fn binary_search(key: usize, v: &Vec<usize>) -> isize {
    let mut left = -1;
    let mut right = (v.len() - 1) as isize;
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

fn bit_partial_sum(sum: usize, v: &Vec<usize>) -> bool {
    let len = v.len();
    let mut exist = false;
    for bit in 0..(1 << len) {
        let bit_list = (0..len)
            .filter(|x| (bit & (1 << x)) != 0)
            .map(|x| v[x as usize]);
        let bit_sum: usize = bit_list.sum();
        if sum == bit_sum {
            exist = true;
        };
    }
    if exist {
        true
    } else {
        false
    }
}

#[test]
fn test_bit_partial_sum() {
    assert_eq!(bit_partial_sum(10, &vec![1, 3, 6]), true);
    assert_eq!(bit_partial_sum(10, &vec![2, 3, 6]), false);
}

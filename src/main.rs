use proconio::input;

fn factorial(n: usize) -> usize {
    if n == 1 {
        n
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    input! {
        mut p: usize,
    };

    let mut ans = 0;
    for i in (1..=10).rev() {
        while p >= factorial(i) {
            ans += 1;
            p -= factorial(i);
        }
    }

    println!("{}", ans);
}

#[test]
fn test_factorial() {
    assert_eq!(factorial(1), 1);
    assert_eq!(factorial(10), 3628800);
}

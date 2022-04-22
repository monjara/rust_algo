use proconio::input;

fn main() {
    input! {
        n: i64,
        a: [u64; n],
        b: [u64; n],
    };
    let mut a_ans = 0;
    let mut b_ans = 0;
    for i in 0..n {
        for j in 0..n {
            if i == j {
                if a[i as usize] == b[j as usize] { a_ans += 1; }
            } else {
                if a[i as usize] == b[j as usize] { 
                    b_ans += 1; 
                    break;
                }
            }

        }
    }
    println!("{}", a_ans);
    println!("{}", b_ans);
}

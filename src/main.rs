use proconio::input;

fn main() {
    input! {
        n: isize,
        w: isize,
        a: [isize; n],
    }
    let mut exist = false;
    for bit in 0..(1 << n) {
        let sub_list = (0..n)
            .filter(|x| (bit & (1 << x)) != 0)
            .map(|x| a[x as usize]);
        println!("");
        let sum: isize = sub_list.sum();
        if w == sum {
            exist = true;
        };
    }
    let ans = if exist { "Yes" } else { "No" };
    println!("{}", ans);
}

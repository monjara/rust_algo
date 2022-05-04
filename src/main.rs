use proconio::input;

fn main() {
    input! {
        n: String,
    }
    if n.len() == 4 {
        println!("{}", n);
    } else {
        let zero_len = 4 - n.len();
        for _i in 0..zero_len {
            print!("0");
        }
        println!("{}", n);
    }
}

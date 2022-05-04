use proconio::input;

fn main() {
    input! {
        s_1: String,
        s_2: String,
        s_3: String,
        t: String,
    }
    for i in t.chars() {
        match i {
            '1' => print!("{}", &s_1),
            '2' => print!("{}", &s_2),
            '3' => print!("{}", &s_3),
            _ => {},
        }
    }
    println!();
}

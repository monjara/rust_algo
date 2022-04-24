use std::{io, str::FromStr};

fn read<T: FromStr>() -> Result<T, T::Err> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim_end().parse()
}

fn read_vector<T: FromStr>() -> Result<Vec<T>, T::Err> {
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("could not read from stdin");
    s.trim()
        .split_whitespace()
        .map(|word| word.parse())
        .collect()
}

fn main() {
    let _n = read::<isize>().unwrap();
    let mut a = read_vector::<isize>().unwrap();
    println!("{:?}", a);
    for i in 0..a.len() {
        let tmp = a[i];
        let mut j = i as isize - 1;
        while j >= 0 && a[j as usize] > tmp {
            a[(j + 1) as usize] = a[j as usize];
            j -= 1;
        }
        a[j as usize + 1] = tmp;
        println!("{:?}", a);
    }
}

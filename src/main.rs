use proconio::input;
use itertools::Itertools;

fn add(a: f32, b:f32)  -> (f32, String, bool) { (a + b, String::from("+"), false) }
fn sub(a: f32, b:f32)  -> (f32, String, bool) { (a - b, String::from("-"), false) }
fn rsub(a: f32, b:f32) -> (f32, String, bool) { (b - a, String::from("-"), true) }
fn mul(a: f32, b:f32)  -> (f32, String, bool) { (a * b, String::from("*"), false) }
fn div(a: f32, b:f32)  -> (f32, String, bool) { (a / b, String::from("/"), false) }
fn rdiv(a: f32, b:f32) -> (f32, String, bool) { (b / a, String::from("/"), true) }


fn output_2(fa: f32, fb: f32, fc: f32, fd: f32, sa: String, sb: String, sc: String) {
    println!("({}{}{}){}({}{}{})", fa, sa, fb, sb, fc, sc, fd);
}

fn main() {
    input! { v: [f32; 4] }
    let calc_funcs: [fn(f32, f32) -> (f32, String, bool); 6] = [add, sub, rsub, mul, div, rdiv];
    // ((a o1 b) o2 c) 03 d
    for perm in v.iter().permutations(2) {
        let mut arr = perm;
        let mut tmp = v.iter().filter(|&el| el != arr[0] && el != arr[1]).collect_vec();
        arr.append(&mut tmp);

        for fn_1 in calc_funcs.iter() {
            for fn_2 in calc_funcs.iter() {
                for fn_3 in calc_funcs.iter() {
                    let (ans_1, str_1, reverse_1) = fn_1(*arr[0], *arr[1]);
                    let (ans_2, str_2, reverse_2) = fn_2(ans_1, *arr[2]);
                    let (ans_3, str_3, reverse_3) = fn_3(ans_2, *arr[3]);
                    if ans_3 == 10.0 {
                        if reverse_1 && reverse_2 && reverse_3 {
                            println!("{}{}({}{}({}{}{}))", *arr[3], str_3, *arr[2], str_2, *arr[1], str_1, *arr[0]);
                        }
                        else if !reverse_1 && reverse_2 && reverse_3 {
                            println!("{}{}({}{}({}{}{}))", *arr[3], str_3, *arr[2], str_2, *arr[0], str_1, *arr[1]);
                        }
                        else if reverse_1 && !reverse_2 && reverse_3 {
                            println!("{}{}(({}{}{}){}{})", *arr[3], str_3, *arr[1], str_1, *arr[0], str_2, *arr[2]);
                        }
                        else if reverse_1 && reverse_2 && !reverse_3 {
                            println!("({}{}({}{}{})){}{}", *arr[2], str_2, *arr[1], str_1, *arr[0], str_3, *arr[3]);
                        }
                        else if reverse_1 && !reverse_2 && !reverse_3 {
                            println!("(({}{}{}){}{}){}{}", *arr[1], str_1, *arr[0], str_2, *arr[2], str_3, *arr[3]);
                        }
                        else if !reverse_1 && reverse_2 && !reverse_3 {
                            println!("({}{}({}{}{})){}{}", *arr[2], str_2, *arr[0], str_1, *arr[1], str_3, *arr[3]);
                        }
                        else if !reverse_1 && !reverse_2 && reverse_3 {
                            println!("{}{}(({}{}{}){}{})", *arr[3], str_3, *arr[0], str_1, *arr[1], str_2, *arr[2]);
                        }
                        else {
                            println!("(({}{}{}){}{}){}{}", *arr[0], str_1, *arr[1], str_2, *arr[2], str_3, *arr[3]);
                        }
                    }
                }
            }
        }
    }
    // (a o1 b) o3 (c o2 d)
    // ab cd
    // ac bd
    // ad cb
    let t = v.to_vec();
    let arrs = [
        [t[0], t[1], t[2], t[3]],
        [t[0], t[2], t[1], t[3]],
        [t[0], t[3], t[2], t[1]],
    ];

    for arr in arrs.iter() {
        for fn_1 in calc_funcs.iter() {
            for fn_2 in calc_funcs.iter() {
                for fn_3 in calc_funcs.iter() {
                    let (ans_1, str_1, reverse_1) = fn_1(arr[0], arr[1]);
                    let (ans_2, str_2, reverse_2) = fn_2(arr[2], arr[3]);
                    let (ans_3, str_3, reverse_3) = fn_3(ans_1, ans_2);
                    if ans_3 == 10.0 {
                        let mut ans_arr = arr.to_vec();

                        if reverse_1 {
                            ans_arr.swap(0, 1);
                        };
                        if reverse_2 {
                            ans_arr.swap(2, 3);
                        };
                        if reverse_3 { 
                            ans_arr.swap(0, 2);
                            ans_arr.swap(1, 3);
                        };

                        let (sa, sb, sc) = if reverse_3 {
                            (str_2, str_3, str_1)
                        } else {
                            (str_1, str_3, str_2)
                        };

                        output_2(ans_arr[0], ans_arr[1], ans_arr[2], ans_arr[3], sa, sb, sc);
                    }
                }
            }
        }
    }
}

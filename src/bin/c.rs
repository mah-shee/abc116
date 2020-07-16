#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: usize,
        mut h: [usize; n]
    }
    let mut ans = 0;
    loop {
        if h.clone().into_iter().max().unwrap() == 0 {
            break;
        }
        let mut i = 0;
        while i < n {
            if h[i] == 0 {
                i += 1;
            } else {
                ans += 1;
                while i < n && h[i] > 0 {
                    h[i] -= 1;
                    i += 1;
                }
            }
        }
    }
    println!("{}", ans);
}

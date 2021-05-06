use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

fn main() {
    // let source = AutoSource::from("0");
    input!{
        // from source,
        n: i64,
        k: i64
    };

    let mut ans = 0i64;

    for ki in k..=n+1 {
        let a = ki * (ki-1) / 2;
        let b = ki * (2*n -ki + 1) / 2;
        ans += (b - a + 1) % 1000000007;
        ans = ans % 1000000007;
    }

    println!("{}", ans);
}

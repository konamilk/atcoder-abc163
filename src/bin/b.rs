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
        m: i64,
        a: [i64; m]
    };

    let mut ans:i64 = n - a.iter().sum::<i64>();

    if ans < 0 {
        ans = -1
    }

    println!("{}", ans);

}

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
        n: usize,
        a: [usize; n-1]
    };

    let mut v = vec![0; n];

    for i in 0..n-1 {
        v[a[i] - 1] += 1
    }

    for x in v {
        println!("{}",x )
    }
}

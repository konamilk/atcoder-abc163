use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
use std::f64::consts::PI;

fn main() {
    // let source = AutoSource::from("0");
    input!{
        // from source,
        r: f64
    };

    println!("{}", 2. * r * PI);
}

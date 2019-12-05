mod lib;
use lib::{increasing, repeat};

fn main() {
    let mut count = 0;
    for i in 153_517..630_395 {
        if repeat(i) && increasing(i) {
            count += 1;
        }
    }
    println!("{}", count);
}

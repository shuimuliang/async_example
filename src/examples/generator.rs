
#![feature(generators, generator_trait)]

use std::ops::{Generator};
use std::pin::Pin;

fn main() {
    let xs = vec![1, 2, 3];
    let mut gen = move|| {
        let mut sum = 0;
        for x in xs {
            sum += x;
            yield sum;
        }
    };
    println!("{:?}",Pin::new(&mut gen).resume());
    println!("{:?}",Pin::new(&mut gen).resume());
    println!("{:?}",Pin::new(&mut gen).resume());
    println!("{:?}",Pin::new(&mut gen).resume());
}

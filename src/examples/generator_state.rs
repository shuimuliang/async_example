#![feature(generators, generator_trait)]

use std::ops::{Generator, GeneratorState};
use std::pin::Pin;

fn main() {
    let xs = vec![1, 2, 3];
    let mut gen = || {
        let mut sum = 0;
        for x in xs.iter() {
            sum += x;
            yield sum;
        }
        for x in xs.iter().rev() {
            sum += x;
            yield sum;
        }
    };

    loop{
        match Pin::new(&mut gen).resume() {
            GeneratorState::Yielded(t) => {
                println!("get {}",t);
            }
            GeneratorState::Complete(()) => {
                println!("finish");
                break;
            }    
        }
    }
}

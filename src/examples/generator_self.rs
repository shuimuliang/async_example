#![feature(generators, generator_trait)]

use std::ops::{Generator};
use std::pin::Pin;

fn main(){
	let mut gen = || {
		let local = 1;
		let ptr=&local;
		yield local;
		yield *ptr;
    };
    println!("{:?}",Pin::new(&mut gen).resume());
    println!("{:?}",Pin::new(&mut gen).resume());    
}
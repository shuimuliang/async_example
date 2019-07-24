#![feature(async_await, impl_trait_in_bindings)]
#[macro_use]
extern crate tokio;

use tokio::runtime::{Runtime};
use tokio::prelude::*;
use std::time::{Duration, Instant};
use tokio::timer::Delay;

use {    
    futures::{
        compat::Future01CompatExt,
        future::{FutureExt, TryFutureExt},
    },
};


async fn delay_task(duration: Duration){
    let timeout_time = Instant::now() + duration;
    if let Err(e) = Delay::new(timeout_time).compat().await {
                println!("Error on delay: {:?}", e);
            };

    println!("hhhh");
}

fn main() {
    let mut rt = Runtime::new().unwrap();

    rt.spawn(
        delay_task(Duration::from_millis(1000))
            .boxed()
            .unit_error()
            .compat(),
    );

    rt.shutdown_on_idle().wait().unwrap();
}
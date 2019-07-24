#![feature(async_await, impl_trait_in_bindings)]
#[macro_use]
extern crate tokio;

use tokio::runtime::{Runtime};
use tokio::prelude::*;
use std::time::{Duration, Instant};
use tokio::timer::Interval;

use {    
    futures::{
        compat::{Compat01As03},
        future::{FutureExt, TryFutureExt},
        stream::{StreamExt},
    },
};


async fn interval_task(duration: Duration){
    let mut ticker= Compat01As03::new(Interval::new(Instant::now(), Duration::from_millis(1000)))
            .fuse();

    while let Some(Ok((s))) = ticker.next().await {
        println!("hhhh");
    }
}

fn main() {
    let mut rt = Runtime::new().unwrap();

    rt.spawn(
        interval_task(Duration::from_millis(1000))
            .boxed()
            .unit_error()
            .compat(),
    );

    rt.shutdown_on_idle().wait().unwrap();
}
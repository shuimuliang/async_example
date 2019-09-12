#![feature(async_await, impl_trait_in_bindings)]
#[macro_use]
extern crate tokio;

use tokio::runtime::{Runtime};
use tokio::prelude::*;
use std::time::{Duration, Instant};
use tokio::timer::Delay;
use std::sync::Arc;

use {    
    futures::{
        compat::Future01CompatExt,
        future::{FutureExt, TryFutureExt},
    },
};

#[derive(Clone)]
struct Wallet {
    lock:Arc<futures_locks::Mutex<u64>>,
}

async fn delay_task(duration: Duration,wallet:Wallet){
    let timeout_time = Instant::now() + duration;
    match Delay::new(timeout_time).compat().await {
        Ok(_)=>{
            println!("start get lock");
            let mut data = wallet.lock.lock().compat().await.unwrap();
            println!("get lock");
            *data+=1;
        },
        Err(_)=>{
            println!("err");
        }
    };

    println!("hhhh");
}

async fn delay_task_sleep(duration: Duration,wallet:Wallet){
    let timeout_time = Instant::now() + duration;
    let mut data = wallet.lock.lock().compat().await.unwrap();
    match Delay::new(timeout_time).compat().await {
        Ok(_)=>{
            println!("lock_delay");
            *data+=1;
        },
        Err(_)=>{
            println!("err");
        }
    };

    println!("hhhh");
}

fn main() {
    let mut rt = Runtime::new().unwrap();

    let wallet = Wallet{
        lock:Arc::new(futures_locks::Mutex::new(0)),
    };

    rt.spawn(
        delay_task_sleep(Duration::from_millis(5000),wallet.clone())
            .boxed()
            .unit_error()
            .compat(),);

    rt.spawn(
        delay_task(Duration::from_millis(1000),wallet.clone())
            .boxed()
            .unit_error()
            .compat(),);
    
    rt.shutdown_on_idle().wait().unwrap();            
}
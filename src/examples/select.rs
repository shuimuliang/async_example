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
        sink::SinkExt,
    },
};

pub type Tx = futures::channel::mpsc::UnboundedSender<String>;

pub type Rx = futures::channel::mpsc::UnboundedReceiver<String>;

async fn interval_task_h(duration: Duration,mut tx:Tx){
    let mut ticker= Compat01As03::new(Interval::new(Instant::now(), Duration::from_millis(1000)))
        .fuse();

    while let Some(Ok((s))) = ticker.next().await {
        tx.send("hhhh".to_string()).await.unwrap();
    }
}

async fn interval_task_g(duration: Duration,mut tx:Tx){
    let mut ticker= Compat01As03::new(Interval::new(Instant::now(), Duration::from_millis(1000)))
        .fuse();

    while let Some(Ok((s))) = ticker.next().await {
        //tx.send("gggg".to_string()).await.unwrap();
        tx.unbounded_send("gggg".to_string());
    }
}

async fn select_recv(mut rx_h:Rx,mut rx_g:Rx){
    loop {
        futures::select! {
            mesg = rx_h.select_next_some() => {
                println!("rx_h message is {:?}",mesg);
            }
            mesg = rx_g.select_next_some() => {
                println!("rx_g message is {:?}",mesg);
            }
        }
    }
}

fn main() {
    let mut rt = Runtime::new().unwrap();

    let (mut tx_h,  rx_h):(Tx,Rx) = futures::channel::mpsc::unbounded();
    let (mut tx_g,  rx_g):(Tx,Rx) = futures::channel::mpsc::unbounded();

    rt.spawn(
        interval_task_h(Duration::from_millis(1000),tx_h)
            .boxed()
            .unit_error()
            .compat(),
    );

    rt.spawn(
        interval_task_g(Duration::from_millis(1000),tx_g)
            .boxed()
            .unit_error()
            .compat(),
    );

    rt.spawn(
        select_recv(rx_h,rx_g)
            .boxed()
            .unit_error()
            .compat(),
    );

    rt.shutdown_on_idle().wait().unwrap();
}
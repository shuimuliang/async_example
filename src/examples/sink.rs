#![feature(async_await, impl_trait_in_bindings)]
use futures::{
    sink::SinkExt, stream::StreamExt,
    future::{FutureExt, TryFutureExt},
    compat::{Stream01CompatExt, Sink01CompatExt, Compat01As03Sink,Compat01As03}
};

use tokio::prelude::*;
use tokio::runtime::{Runtime};
use tokio::sync::mpsc::{channel,Sender,Receiver};

fn main(){
    let mut rt = Runtime::new().unwrap();

    let (tx, rx) = channel(1);
    let (mut tx, mut rx) = (tx.sink_compat(), rx.compat());

    rt.spawn(send(tx)
            .boxed()
            .unit_error()
            .compat(),
    );

    rt.spawn(recv(rx)
                 .boxed()
                 .unit_error()
                 .compat(),
    );

    rt.shutdown_on_idle().wait().unwrap();
}

async fn send(mut tx: Compat01As03Sink<Sender<i32>,i32>){

    tx.send(1).await.unwrap();
    drop(tx);
}

async fn recv(mut rx:Compat01As03<Receiver<i32>>) {
    println!("{:?}",rx.next().await);
    println!("{:?}",rx.next().await);
}
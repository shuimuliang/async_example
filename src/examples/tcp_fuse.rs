#![feature(async_await, impl_trait_in_bindings)]
#[macro_use]
extern crate tokio;

use tokio::net::{TcpListener, TcpStream};
 use tokio::net::tcp::Incoming;
use tokio::runtime::{Runtime,TaskExecutor};
use futures_01::future::Future as Future01; //需要重命名，否则冲突 rt.shutdown_on_idle().wait().unwrap(); 编译过不了
use tokio::codec::{LinesCodec, Decoder, Framed};
use tokio::prelude::*;

use {
    futures::{
        compat::{Compat01As03},
        future::{FutureExt, TryFutureExt},
        stream::{Stream,Fuse,StreamExt,FuturesUnordered},
        io::{AsyncWriteExt,AsyncReadExt},
    },
    std::net::SocketAddr,
};

async fn handle(mut executor:TaskExecutor ,mut server_listener:Incoming){
    let mut listener = Compat01As03::new(server_listener).fuse();
    loop {
        futures::select! {
            incoming_connection = listener.select_next_some() => {
                executor.spawn(handle_connection(incoming_connection.unwrap()).boxed().unit_error().compat());
            }
        }
    }
}

async fn handle_connection(connection :TcpStream){
    let mut f_stream = Compat01As03::new(Framed::new(connection, LinesCodec::new())).fuse();

    loop{
        futures::select! {
            data = f_stream.select_next_some() => {
                println!("{}",data.unwrap());
            }
        }
    }
}

fn main() {
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let mut rt = Runtime::new().unwrap();

    let incoming=listener.incoming();
    let executor = rt.executor();

    rt.spawn(
        handle(executor,incoming)
            .boxed()
            .unit_error()
            .compat(),
    );

    rt.shutdown_on_idle().wait().unwrap();
}

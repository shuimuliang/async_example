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
        stream::{StreamExt},
        io::{AsyncWriteExt,AsyncReadExt},
    },
    std::net::SocketAddr,
};


async fn handle(mut executor:TaskExecutor ,mut server_listener:Compat01As03<Incoming>)
{
    while let Some(Ok((f_stream))) = server_listener.next().await {
        println!("{:?}",f_stream);
        let mut sock=Compat01As03::new(f_stream);

        let mut output = [0u8; 20];
        loop{
            let bytes = sock.read(&mut output[..]).await.unwrap();
            println!("data is {:?}",std::str::from_utf8(&output[0..bytes]));
            sock.write_all(b"hello\r\n").await.unwrap();
        }
    }    
}

fn main() {
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let mut rt = Runtime::new().unwrap();

    let incoming=Compat01As03::new(listener.incoming());
    let executor = rt.executor();

    rt.spawn(
        handle(executor,incoming)
            .boxed()
            .unit_error()
            .compat(),
    );

    rt.shutdown_on_idle().wait().unwrap();
}
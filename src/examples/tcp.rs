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
        compat::{Compat01As03,Sink01CompatExt,Stream01CompatExt},
        future::{FutureExt, TryFutureExt},
        stream::{StreamExt},
        sink::SinkExt,
    },
    std::net::SocketAddr,
};
 use tokio::prelude::stream::{SplitSink};


 async fn handle(mut executor:TaskExecutor ,mut server_listener:Compat01As03<Incoming>)
{
    while let Some(Ok((f_stream))) = server_listener.next().await {
        let (mut framed_sink,framed_stream) = LinesCodec::new().framed(f_stream).split();
        let mut compat_stream = framed_stream.compat();

        executor.spawn(handle_send(framed_sink).boxed()
            .unit_error()
            .compat());

        while let Some(Ok(data)) = compat_stream.next().await {
            println!("data is {:?}",data);
        }
    }    
}

async fn handle_send(mut framed_sink:SplitSink<Framed<TcpStream,LinesCodec>>) {
    let mut compat_sink = framed_sink.sink_compat();
    compat_sink.send("hello".to_string()).await.unwrap();
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
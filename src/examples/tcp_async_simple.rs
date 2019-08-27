use std::{
    net::ToSocketAddrs,
    sync::Arc,
};

use futures::{
    channel::mpsc,
};

use async_std::{
    io::{BufReader},
    prelude::*,
    task,
    net::{TcpListener, TcpStream},
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
    task::block_on(server("127.0.0.1:8080"))
}

async fn server(addr: impl ToSocketAddrs) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;

    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        println!("Accepting from: {}", stream.peer_addr()?);

        task::spawn(handle_stream(stream));
    }
    Ok(())
}

async fn handle_stream(stream: TcpStream) -> Result<()> {
    let stream_arc = Arc::new(stream);

    let reader_arc_clone = stream_arc.clone();

    let (tx,mut rx) = mpsc::unbounded();

    let reader_future = async move {
        let reader = BufReader::new(&*reader_arc_clone);
        let mut lines = reader.lines();

        while let Some(line) = lines.next().await {
            let line = line.unwrap();
            println!("data is {}",line);
            tx.unbounded_send(line).unwrap();
        };
    };

    let writer_clone = stream_arc.clone();
    let writer_future = async move {
        let mut stream = &*writer_clone;
        while let Some(msg) = rx.next().await{
            stream.write_all(msg.as_bytes()).await.unwrap();
            stream.write_all(b"\r\n").await.unwrap();
        }
    };

    task::spawn(reader_future);
    task::spawn(writer_future);
    Ok(())
}

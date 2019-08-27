use std::{
    net::ToSocketAddrs,
};


use async_std::{
    io,
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
    let (reader, writer) = &mut (&stream, &stream);
    io::copy(reader, writer).await?;
    Ok(())
}

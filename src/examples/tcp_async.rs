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
type Sender<T> = mpsc::UnboundedSender<T>;
type Receiver<T> = mpsc::UnboundedReceiver<T>;

fn main() -> Result<()> {
    task::block_on(server("127.0.0.1:8080"))
}

async fn server(addr: impl ToSocketAddrs) -> Result<()> {
    let listener = TcpListener::bind(addr).await?;

    let (stream_tx,stream_rx) = mpsc::unbounded();
    task::spawn(handle_write(stream_rx));
    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream?;
        println!("Accepting from: {}", stream.peer_addr()?);

        task::spawn(handle_read_stream(stream,stream_tx.clone()));
    }
    Ok(())
}

async fn handle_read_stream(stream: TcpStream,stream_sender:Sender<(Arc<TcpStream>,Receiver<String>)>) -> Result<()> {
    let stream = Arc::new(stream);
    let reader = BufReader::new(&*stream);
    let mut lines = reader.lines();

    let (msg_tx,msg_rx) = mpsc::unbounded();

    stream_sender.unbounded_send((stream.clone(),msg_rx))?;

    while let Some(line) = lines.next().await {
        let line = line?;
        println!("data is {}",line);
        msg_tx.unbounded_send(line)?;
    };

    Ok(())
}

async fn handle_write(mut stream_receiver:Receiver<(Arc<TcpStream>,Receiver<String>)>) -> Result<()> {
    while let Some(stream) = stream_receiver.next().await{
        let (stream,rx) = stream;
        task::spawn(handle_write_stream(stream,rx));
    };
    Ok(())
}

async fn handle_write_stream(stream:Arc<TcpStream>,mut rx:Receiver<String>){
    let mut stream = &*stream;
    while let Some(msg) = rx.next().await{
        stream.write_all(msg.as_bytes()).await.unwrap();
        stream.write_all(b"\r\n").await.unwrap();
    }
}

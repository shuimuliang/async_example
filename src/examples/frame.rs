use bytes::BytesMut;
use futures::{
    future::Future,
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
};
use std::{convert::TryInto, io::Result};

pub async fn read_u16frame<TSocket>(mut sock:TSocket) -> Vec<u8> 
where
    TSocket: AsyncRead + AsyncWrite + Unpin,
{
    let mut len_buf = [0, 0];
    sock.read_exact(&mut len_buf).await.unwrap();

    let len=u16::from_be_bytes(len_buf);

    let mut buf = BytesMut::new();
    buf.resize(len as usize, 0);
    sock.read_exact(buf.as_mut()).await.unwrap();
    buf.to_vec()
}

pub async fn write_u16frame<TSocket>(mut sock:TSocket,data:&[u8])
where
    TSocket: AsyncRead + AsyncWrite + Unpin,
{
    let len = u16::to_be_bytes(data.len() as u16);
    sock.write_all(&len).await.unwrap();
    sock.write_all(data).await.unwrap();
}
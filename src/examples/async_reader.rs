#![feature(async_await, impl_trait_in_bindings)]
use futures::io::AsyncReadExt;
use futures::compat::AsyncRead01CompatExt;
use futures::future::{FutureExt, TryFutureExt};

use tokio::prelude::*;
use tokio::runtime::{Runtime};

fn main(){
    let mut rt = Runtime::new().unwrap();

    rt.spawn(read()
            .boxed()
            .unit_error()
            .compat(),
    );

    rt.shutdown_on_idle().wait().unwrap();
}

async fn read(){
    let input = b"Hello World!";
    let reader: impl AsyncRead = std::io::Cursor::new(input);
    let mut reader: impl futures::io::AsyncRead + Unpin = reader.compat();

    let mut output = Vec::with_capacity(12);
    reader.read_to_end(&mut output).await.unwrap();
    println!("output is {:?}",std::str::from_utf8(output.as_slice()).unwrap());
}
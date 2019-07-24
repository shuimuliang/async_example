#![feature(async_await, impl_trait_in_bindings)]
use futures::io::AsyncWriteExt;
use futures::compat::AsyncWrite01CompatExt;
use futures::future::{FutureExt, TryFutureExt};

use tokio::prelude::*;
use tokio::runtime::{Runtime};

fn main(){
    let mut rt = Runtime::new().unwrap();

    rt.spawn(write()
            .boxed()
            .unit_error()
            .compat(),
    );

    rt.shutdown_on_idle().wait().unwrap();
}

async fn write(){
    let input = b"Hello World!";
    let mut cursor = std::io::Cursor::new(Vec::with_capacity(12));

    let mut writer = (&mut cursor).compat();
    writer.write_all(input).await.unwrap();
    assert_eq!(cursor.into_inner(), input);
}
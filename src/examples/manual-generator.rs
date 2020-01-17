#![feature(
    async_await,
    futures_api,
    gen_future,
    generator_trait,
    generators,
    maybe_uninit,
    pin
)]

use core::future::Future;

pub mod io {
    use core::future::Future;

    pub struct AsyncRead(Vec<u8>);

    impl AsyncRead {
        pub fn new(data: Vec<u8>) -> AsyncRead {
            AsyncRead(data)
        }

        pub fn read_to_end(&mut self) -> impl Future<Output = Vec<u8>> + '_ {
            async move { self.0.clone() }
        }
    }
}

use self::io::AsyncRead;
use async_std::task;

pub fn quote_encrypt_unquote(data: &mut AsyncRead) -> impl Future<Output = Vec<u8>> + '_{
    use core::{pin::Pin, task::Poll};
    use std::future::{poll_with_tls_context,from_generator};
        
    from_generator(static move || {
        let mut pad = AsyncRead::new(vec![4; 32]);
        let data = {
            let mut pinned = data.read_to_end();
            loop {
                if let Poll::Ready(x) =
                poll_with_tls_context(unsafe { Pin::new_unchecked(&mut pinned) })
                {
                    break x;
                }
                yield
            }
        };
        let pad = {
            let mut pinned = pad.read_to_end();
            loop {
                if let Poll::Ready(x) =
                poll_with_tls_context(unsafe { Pin::new_unchecked(&mut pinned) })
                {
                    break x;
                }
                yield
            }
        };
        data.into_iter().zip(pad).map(|(a, b)| a ^ b).collect() 
    })   
}

fn main() {
    let mut data = AsyncRead::new("hello".into());
    let encrypted = task::block_on(quote_encrypt_unquote(&mut data));
    println!("Encrypted: {}", core::str::from_utf8(&encrypted).unwrap());
}
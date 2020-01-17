#![feature(
    gen_future,
    generator_trait,
    generators,
)]

use hyper::{body::HttpBody as _, Client};
use anyhow::*;
use tokio::runtime::Runtime;
use core::{pin::Pin, task::Poll};
use std::future::{poll_with_tls_context,from_generator};
use core::future::Future;

fn fetch_url(url: hyper::Uri) -> impl Future<Output = Result<bytes::Bytes>> {
    async {
        let client = Client::new();

        let mut res = client.get(url).await?;

        println!("Response: {}", res.status());
        println!("Headers: {:#?}\n", res.headers());

        match res.data().await {
            Some(t)=>{
                return Ok(t?);
            }
            None=>{
                bail!("error");
            }
        }
    }
}

fn fetch_two_url(url1:hyper::Uri,url2:hyper::Uri)->impl Future<Output = Result<()>>{
    
    from_generator(static move || {
        let result1 = {
            let mut future1 = fetch_url(url1);
            loop {
                if let Poll::Ready(x) =
                poll_with_tls_context( unsafe {Pin::new_unchecked(&mut future1)})
                {
                    break x;
                }
                yield
            }
            
        };
        let result2 = {
            let mut future2 = fetch_url(url2);
            loop {
                if let Poll::Ready(x) =
                poll_with_tls_context(unsafe { Pin::new_unchecked(&mut future2) })
                {
                    break x;
                }
                yield
            }
            
        };
        println!("result 1 is {:?},result 2 is {:?}",result1,result2);
        Ok(())
    })
}

fn main() {
    let mut rt = Runtime::new().unwrap();

    let url1 = "http://www.baidu.com".parse::<hyper::Uri>().unwrap();
    let url2 = "http://cn.bing.com".parse::<hyper::Uri>().unwrap();
    rt.block_on(fetch_two_url(url1,url2)).unwrap();
}
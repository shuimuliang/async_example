#![feature(
    gen_future,
    generator_trait,
    generators,
)]

use hyper::{body::HttpBody as _, Client};
use anyhow::*;
use tokio::runtime::Runtime;

async fn fetch_url(url: hyper::Uri) -> Result<bytes::Bytes> {
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

async fn fetch_two_url(url1:hyper::Uri,url2:hyper::Uri)->Result<()>{
    let result1=fetch_url(url1).await?;
    let result2=fetch_url(url2).await?;

    println!("result 1 is {:?},result 2 is {:?}",result1,result2);
    Ok(())
}

fn main() {
    let mut rt = Runtime::new().unwrap();

    let url1 = "http://www.baidu.com".parse::<hyper::Uri>().unwrap();
    let url2 = "http://cn.bing.com".parse::<hyper::Uri>().unwrap();
    rt.block_on(fetch_two_url(url1,url2)).unwrap();
}
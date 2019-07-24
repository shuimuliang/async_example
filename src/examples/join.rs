#![feature(async_await, impl_trait_in_bindings)]
#[macro_use]
extern crate tokio;

use {    
    futures::{
        executor::block_on,
    },
};

struct Song {}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn dance() {
    println!("dance");
}

async fn learn_song() -> Song {
    println!("learn song");
    Song{}
}

async fn sing_song(song: Song) {
    println!("sing song");
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    futures::join!(f1, f2);
}

fn main() {
    block_on(async_main());
}
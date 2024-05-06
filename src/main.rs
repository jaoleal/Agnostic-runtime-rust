use std::{future::Future, process::Output};
use anyhow::Result;
use std::marker::Send;
use async_std::task::{self as std_task};
use tokio::task::{spawn as tokio_spawn};


trait Asyncfunctions {
    async fn task<F, T>(&self,t: F) -> T where  F: Future<Output = T> + Send + 'static,
    T: Send + 'static;
}
struct Stdeisync;

struct TokioRuntime; 

impl Asyncfunctions for TokioRuntime{
    async fn task<F, T>(&self,t: F) -> T  where  F: Future<Output = T> + Send + 'static,
    T: Send + 'static{
        tokio_spawn(t).await.unwrap()   
    }
}
impl Asyncfunctions for Stdeisync{
    async fn task<F, T>(&self,t: F) -> T where  F: Future<Output = T> + Send + 'static,
    T: Send + 'static{
        std_task::spawn(t).await
    }
}

async fn agnostic_function<F: Asyncfunctions> (runtime: F) -> Result<()> {
    let task = runtime.task( async {
            let mut i = 0;
            for j in 0..1_000_000_000 {
                i += 1;
            }
            println!("one billion is reached. i:{}", i );
    });
    task.await;
    Ok(())
}
#[tokio::main]
async fn main() {

    println!("print one billion using Async-std funtions:");
    let _ =  agnostic_function(Stdeisync).await;
    println!("print one billion using tokio functions:");
    let _ =  agnostic_function(TokioRuntime).await;

}

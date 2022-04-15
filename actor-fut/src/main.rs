use std::time::Duration;

use actix::{clock, Actor, Context, Handler, Message, ResponseFuture};
use chrono::Utc;

// Messages
#[derive(Debug)]
struct Ping(usize);

impl Message for Ping {
    type Result = ();
}

// Actors
struct A;

impl Actor for A {
    type Context = Context<Self>;
}

impl Handler<Ping> for A {
    type Result = ResponseFuture<()>;

    fn handle(&mut self, msg: Ping, _ctx: &mut Self::Context) -> Self::Result {
        println!("{:?} START {:?}", Utc::now(), msg);

        // return a future and let the actix runtime execute it
        Box::pin(async move {
            do_async_work().await;
            println!("{:?} DONE {:?}", Utc::now(), msg)
        })
    }
}

async fn do_async_work() {
    clock::sleep(Duration::from_secs(1)).await;
}

#[actix::main]
async fn main() -> anyhow::Result<()> {
    let a = A.start();
    a.do_send(Ping(0));
    a.send(Ping(1)).await?;

    Ok(())
}

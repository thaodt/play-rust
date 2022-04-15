use color_eyre::Report;
use tracing::info;
use tracing_subscriber::EnvFilter;
use reqwest::Client;

// mod dumb;

pub const URL_1: &str = "https://fasterthanli.me/articles/whats-in-the-box";
pub const URL_2: &str = "https://fasterthanli.me/series/advent-of-code-2020/part-13";

// fn type_name_of<T>(_: &T) -> &'static str {
//     std::any::type_name::<T>()
// }

#[tokio::main]
async fn main()  -> Result<(), Report> {
    setup()?;

    // info!("Building that fetch future...");
    let client = Client::new();
    // let url = "https://fasterthanli.me";
    // // this will turn non-200 HTTP status codes into rust errors,
    // // so the first `?` propagates "we had a connection problem" and
    // // the second `?` propagates "we had a chat with the server and they
    // // were not pleased"
    // let res = client.get(url).send().await?.error_for_status()?;
    // info!(%url, content_type = ?res.headers().get("content-type"), "Got a response!");

    // let fut1 = fetch_thing(&client, URL_1); // panic: &client borrowed value does not live long enough, fetch_thing argument requires that `client` is borrowed for `'static`
    // let fut2 = fetch_thing(&client, URL_2);
    // fut1.await?;
    // fut2.await?;

    let leaked_client = Box::leak(Box::new(client));

    let fut1 = fetch_thing(leaked_client, URL_1);
    let fut2 = fetch_thing(leaked_client, URL_2);

    let handle1 = tokio::spawn(fut1);
    let handle2 = tokio::spawn(fut2);

    handle1.await.unwrap()?;
    handle2.await.unwrap()?;

    
    
    // dumb mod
    // info!("Building that dumb future...");
    // let fut = dumb::DumbFuture {};
    // info!("Awaiting that dumb future...");
    // fut.await;
    // info!("Done awaiting that dumb future");

    // -----------------------------------------

    // let fut = fetch_thing(&client, URL_1);
    // info!("Sleeping for a bit...");
    // sleep(Duration::from_secs(1)).await;
    // // info!(
    // //     type_name = type_name_of(&fut),
    // //     "That fetch future has a type.."
    // // );
    // info!("Awaiting that fetch future...");
    // fut.await?;
    // info!("Done awaiting that fetch future");

    Ok(())  
}

// async fn fetch_thing(client: &Client, url: &str) -> Result<(), Report> {
//     let res = client.get(url).send().await?.error_for_status()?;
//     info!(%url, content_type = ?res.headers().get("content-type"), "Got a response!");
//     Ok(())
// }

use std::future::Future;

// the crunchy-core way: that it's a regular "fn" that just happens to return a future
fn fetch_thing(
    client: &'static Client,
    url: &'static str,
) -> impl Future<Output = Result<(), Report>> + 'static {
    async move {
        let res = client.get(url).send().await?.error_for_status()?;
        info!(%url, content_type = ?res.headers().get("content-type"), "Got a response!");
        Ok(())
    }
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }
    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}
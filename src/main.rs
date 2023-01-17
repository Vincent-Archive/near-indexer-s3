use anyhow::Result;
use dotenv::dotenv;
use futures::join;
use near_indexer_s3::config::{init_db_pool, init_tracing};
use near_indexer_s3::indexer::stream::indexer_stream_from_s3;
use near_indexer_s3::server::http::services;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    init_tracing();

    init_db_pool().await;

    let services = services();
    let indexer = indexer_stream_from_s3();
    join!(services, indexer);

    Ok(())
}

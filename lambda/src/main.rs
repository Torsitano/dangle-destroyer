use lambda_runtime::{service_fn, Error, LambdaEvent};
use log::LevelFilter;
use serde::Serialize;
use serde_json::Value;
use simple_logger::SimpleLogger;

use crate::aws::Route53;
//use std::env;

pub mod aws;

#[derive(Serialize)]
struct Response {
    code: String,
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .with_utc_timestamps()
        .init()
        .unwrap();

    let handler = service_fn(handler);
    lambda_runtime::run(handler).await?;
    Ok(())
}

async fn handler(_event: LambdaEvent<Value>) -> Result<(), Error> {
    let r53 = Route53::new().await;

    let domains = r53.get_all_hosted_zones().await?;

    log::info!("{:?}", domains);

    Ok(())
}

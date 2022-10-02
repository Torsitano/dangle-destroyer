use aws_sdk_route53;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use log::LevelFilter;
use serde::Serialize;
use serde_json::Value;
use simple_logger::SimpleLogger;
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
    let config = aws_config::load_from_env().await;

    let r53_client = aws_sdk_route53::Client::new(&config);

    let domains = aws::get_all_hosted_zones(&r53_client).await?;

    log::info!("{:?}", domains);

    Ok(())
}

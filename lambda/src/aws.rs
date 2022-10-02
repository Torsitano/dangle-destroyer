use aws_sdk_route53::model::HostedZone;
//use aws_sdk_route53::{Client, Error};
use tokio_stream::StreamExt;

// pub async fn get_domains(client: &aws_sdk_route53::Client)
// //-> Result<Vec<String>, aws_sdk_route53::Error>
// {
// }

pub async fn get_all_hosted_zones(
    client: &aws_sdk_route53::Client,
) -> Result<Vec<HostedZone>, aws_sdk_route53::Error> {
    let mut hosted_zones: Vec<HostedZone> = vec![];

    let mut hz_paginator = client.list_hosted_zones().into_paginator().items().send();

    while let Some(item) = hz_paginator.try_next().await? {
        hosted_zones.push(item);
    }

    Ok(hosted_zones)
}

use aws_sdk_route53::model::{HostedZone, ResourceRecordSet, RrType};
use tokio_stream::StreamExt;

pub struct Route53 {
    client: aws_sdk_route53::Client,
}

impl Route53 {
    pub async fn new() -> Route53 {
        let config = aws_config::load_from_env().await;

        Route53 {
            client: aws_sdk_route53::Client::new(&config),
        }
    }

    pub async fn get_all_hosted_zones(&self) -> Result<Vec<HostedZone>, aws_sdk_route53::Error> {
        let mut hosted_zones: Vec<HostedZone> = vec![];

        let mut hz_paginator = self
            .client
            .list_hosted_zones()
            .into_paginator()
            .items()
            .send();

        while let Some(item) = hz_paginator.try_next().await? {
            hosted_zones.push(item);
        }

        Ok(hosted_zones)
    }

    pub async fn get_record_sets_for_zone(
        &self,
        hosted_zone: &HostedZone,
    ) -> Result<Vec<ResourceRecordSet>, aws_sdk_route53::Error> {
        let mut record_sets: Vec<ResourceRecordSet> = vec![];

        let id = hosted_zone.id().map(str::to_string);

        let mut response = self
            .client
            .list_resource_record_sets()
            .set_hosted_zone_id(id.clone())
            .send()
            .await?;

        record_sets.append(
            &mut response
                .resource_record_sets()
                .unwrap()
                .iter()
                .filter(|record| match record.r#type() {
                    Some(RrType::A) => true,
                    _ => false,
                })
                .cloned()
                .collect(),
        );

        while response.is_truncated() == true {
            response = self
                .client
                .list_resource_record_sets()
                .set_hosted_zone_id(id.clone())
                .set_start_record_type(response.next_record_type().map(|r| r.clone()))
                .set_start_record_identifier(response.next_record_identifier().map(str::to_string))
                .set_start_record_name(response.next_record_name().map(str::to_string))
                .send()
                .await?;

            record_sets.append(
                &mut response
                    .resource_record_sets()
                    .unwrap()
                    .iter()
                    .filter(|record| match record.r#type() {
                        Some(RrType::A) => true,
                        _ => false,
                    })
                    .cloned()
                    .collect(),
            );
        }

        Ok(record_sets)
    }
}

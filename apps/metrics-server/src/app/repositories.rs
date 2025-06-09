use std::sync::Arc;
use clickhouse::Client;
use serde::Deserialize;
use crate::feature::metrics::repository::MetricRepository;

#[derive(Clone)]
pub struct  Repositories{
    pub metrics_repositories: Arc<MetricRepository>,
}

impl Repositories{
    pub fn new(chouse : Arc<Client>) -> Self{
        Self{
            metrics_repositories: Arc::new(MetricRepository::new(chouse.clone())),
        }
    }
}
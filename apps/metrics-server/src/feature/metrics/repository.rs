use clickhouse::Client;
use std::sync::Arc;

pub struct MetricRepository {
    chouse: Arc<Client>,
}

impl MetricRepository {
    pub fn new(chouse: Arc<Client>) -> Self {
        Self { chouse }
    }
}


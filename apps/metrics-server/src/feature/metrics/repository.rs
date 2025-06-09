use std::sync::Arc;
use clickhouse::Client;

pub struct MetricRepository{
    chouse: Arc<Client>,
}

impl MetricRepository{
    pub fn new(chouse : Arc<Client>) -> Self{
        Self{chouse}
    }
}
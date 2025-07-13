use clickhouse::Client;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone)]
pub struct Repositories {}

impl Repositories {
    pub fn new(chouse: Arc<Client>) -> Self {
        Self {}
    }
}

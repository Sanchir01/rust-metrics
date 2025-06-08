use std::sync::Arc;
use clickhouse::{Client, Row};
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub async fn connect_click_house(
    url: &str,
    db: &str,
    password: &str,
    user: &str,
) -> Result<Arc<Client>> {
    let client = Client::default()
        .with_url(url)
        .with_user(user)
        .with_password(password)
        .with_database(db);


    let rows = client.query("SELECT 1").fetch_all::<SimpleRow>().await?;
    if rows.is_empty() {
        anyhow::bail!("ClickHouse returned empty result");
    }

    Ok(Arc::new(client))
}

#[derive(Row,Deserialize,Serialize,Debug)]
struct SimpleRow {
    #[allow(dead_code)]
    one: u8,
}

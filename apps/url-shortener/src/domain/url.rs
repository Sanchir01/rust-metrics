use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct Url {
    pub id: Uuid,
    pub alias: String,
    pub url: String,
}

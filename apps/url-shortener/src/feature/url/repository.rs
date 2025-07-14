use async_trait::async_trait;
use mockall::{automock, predicate::*};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait UrlRepositoryTrait {
    async fn get_all_url(&self) -> Result<Uuid, sqlx::Error>;
}

#[derive(Clone)]
pub struct UrlRepository {
    primary_db: Pool<Postgres>,
}

impl UrlRepository {
    pub fn new_url_repository(primary_db: Pool<Postgres>) -> Self {
        Self { primary_db }
    }
}

impl UrlRepositoryTrait for UrlRepository {}

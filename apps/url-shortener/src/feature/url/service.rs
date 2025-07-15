use std::sync::Arc;
use async_trait::async_trait;
use mockall::automock;
use crate::domain::url::Url;
use crate::feature::url::repository::{UrlRepository, UrlRepositoryTrait};

#[cfg_attr(test, automock)]
#[async_trait]
pub trait UrlServiceTrait:Send + Sync{
    async fn get_all_url(&self) -> Result<Vec<Url>, sqlx::Error>;
}
#[derive(Clone)]
pub struct UrlService {
    url_repository: Arc<UrlRepository>,
}

impl UrlService {
    pub fn new(url_repository: Arc<UrlRepository>) -> Self {
        Self { url_repository }
    }
}

#[async_trait]
impl UrlServiceTrait for  UrlService {
    async fn get_all_url(&self) -> Result<Vec<Url>, sqlx::Error>{
        self.url_repository.get_all_url().await
    }
}
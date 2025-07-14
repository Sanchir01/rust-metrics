use async_trait::async_trait;
use mockall::{automock, predicate::*};
use sea_query::{PostgresQueryBuilder, Query};
use sqlx::{Pool, Postgres, query_as, query_as_with};

use crate::domain::url::Url;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait UrlRepositoryTrait{
    async fn get_all_url(&self) -> Result<Vec<Url>, sqlx::Error>;
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

// План:
 // 1. Исправить объявление async fn get_all_url, чтобы сигнатура совпадала с trait (ошибка с lifetime).
 // 2. Исправить вызов Query::select().build, чтобы возвращаемые значения корректно использовались.
 // 3. Убрать неиспользуемую переменную _value для соответствия линтеру.
#[async_trait]
impl UrlRepositoryTrait for UrlRepository {
   async fn get_all_url(&self) -> Result<Vec<Url>, sqlx::Error>{
        let (sql, _) = Query::select()
            .columns(["id", "alias", "url"])
            .from("url")
            .build(PostgresQueryBuilder);
        let urls: Vec<Url> = query_as::<_, Url>(&sql)
            .fetch_all(&self.primary_db)
            .await
            .map_err(|err| {
                eprintln!("❌ Error fetching urls: {:?}", err);
                err
            })?;
        Ok(urls)
    }
}

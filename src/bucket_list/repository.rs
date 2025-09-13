use crate::bucket_list::model::{AddToBucketListValidated, BucketListItem};
use crate::common::context::{Context, ContextError, FromContext};
use crate::common::db::SqliteClient;
use error_stack::{Report, ResultExt};
use poem::error::ResponseError;
use poem::http::StatusCode;
use rusqlite::{Connection, named_params};
use std::sync::MutexGuard;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BucketListRepositoryError {
    #[error("Query error")]
    QueryError,
    #[error("Row Value error")]
    RowValueError,
    #[error("Lock error")]
    LockError,
}

impl ResponseError for BucketListRepositoryError {
    fn status(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub struct BucketListRepository {
    sqlite_client: SqliteClient,
}

impl BucketListRepository {
    pub fn new(sqlite_client: SqliteClient) -> Self {
        Self { sqlite_client }
    }

    pub fn get_all_from_bucket_list(
        &self,
    ) -> Result<Box<[BucketListItem]>, Report<BucketListRepositoryError>> {
        let conn = self.borrow_conn()?;

        let mut stmt = conn
            .prepare_cached(include_str!("_sql/get_all_from_bucket_list.sql"))
            .change_context(BucketListRepositoryError::QueryError)?;

        let item_iter = stmt
            .query_map([], |row| {
                Ok(BucketListItem {
                    id: row.get("id")?,
                    name: row.get("name")?,
                    description: row.get("description")?,
                    timestamp: row.get("timestamp")?,
                })
            })
            .change_context(BucketListRepositoryError::RowValueError)?;

        let mut items: Vec<BucketListItem> = Vec::new();
        for item in item_iter {
            items.push(item.change_context(BucketListRepositoryError::RowValueError)?);
        }

        Ok(items.into())
    }

    pub fn add_to_bucket_list(
        &self,
        add_to_bucket_list: &AddToBucketListValidated,
    ) -> Result<(), Report<BucketListRepositoryError>> {
        let conn = self.borrow_conn()?;

        conn.execute(
            include_str!("_sql/add_to_bucket_list.sql"),
            named_params! {
                ":name": add_to_bucket_list.name.as_str(),
                ":description": add_to_bucket_list.description.as_str(),
            },
        )
        .change_context(BucketListRepositoryError::QueryError)?;

        Ok(())
    }

    fn borrow_conn(
        &'_ self,
    ) -> Result<MutexGuard<'_, Connection>, Report<BucketListRepositoryError>> {
        let guard = self
            .sqlite_client
            .get_conn()
            .lock()
            .map_err(|_| Report::new(BucketListRepositoryError::LockError))?;
        Ok(guard)
    }
}

impl FromContext for BucketListRepository {
    async fn from_context(ctx: &'_ Context<'_>) -> Result<Self, Report<ContextError>> {
        Ok(Self::new(ctx.inject().await?))
    }
}

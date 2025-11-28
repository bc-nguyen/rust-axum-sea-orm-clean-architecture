use std::{future::Future, pin::Pin, sync::Arc};

use sea_orm::{DatabaseConnection, DatabaseTransaction, TransactionTrait};

use crate::{domain::error::DomainError, infrastructure::db::RepositoryProvider};

#[derive(Clone, Debug)]
pub struct DbContext {
    conn: Arc<DatabaseConnection>,
}

impl DbContext {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    pub async fn transaction<T, F>(&self, f: F) -> Result<T, DomainError>
    where
        F: for<'a> FnOnce(
            &'a RepositoryProvider<DatabaseTransaction>,
        )
            -> Pin<Box<dyn Future<Output = Result<T, DomainError>> + Send + 'a>>,
        T: Send,
    {
        let tx = self.conn.begin().await?;

        let provider = RepositoryProvider::new(&tx);

        match f(&provider).await {
            Ok(value) => {
                tx.commit().await?;
                Ok(value)
            }
            Err(err) => {
                tx.rollback().await?;
                Err(err)
            }
        }
    }

    pub fn provider(&self) -> RepositoryProvider<'_, DatabaseConnection> {
        RepositoryProvider::new(&self.conn)
    }
}

#[macro_export]
macro_rules! with_transaction {
    ($db_context:expr, $provider:ident => $body:block) => {
        $db_context
            .transaction(|$provider| Box::pin(async move { $body }))
            .await
    };
}

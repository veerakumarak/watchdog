use bb8::{Pool, PooledConnection};
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use crate::errors::AppError;

pub type PgPool = Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

pub type DbConnection<'a> = PooledConnection<'a, AsyncDieselConnectionManager<AsyncPgConnection>>;

pub async fn get_connection_pool(db_url: &str) -> Result<PgPool, AppError> {
    let connection_manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);
    let pool = Pool::builder().build(connection_manager).await?;
    Ok(pool)
}

use crate::db::connection::PgPool;
use crate::db::settings_repository::get_settings;
use crate::errors::AppError;
use crate::models::Settings;

pub async fn from_db(pool: &PgPool) -> Result<Settings, AppError> {
    let mut conn = pool.get().await?;
    let _settings = get_settings(&mut conn).await?;
    Ok(_settings)
}

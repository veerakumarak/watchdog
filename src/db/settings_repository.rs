use diesel::{QueryDsl, ExpressionMethods};
use diesel_async::RunQueryDsl;
use crate::db::connection::DbConnection;
use crate::errors::AppError;
use crate::models::{Settings};

pub async fn get_settings(
    conn: &mut DbConnection<'_>,
) -> Result<Settings, AppError> {
    use crate::schema::global_settings::dsl::*;
    let res = global_settings
        .filter(id.eq(1))
        .first::<Settings>(conn)
        .await
        .expect("Error loading settings");
    Ok(res)
}

pub async fn save_settings(
    conn: &mut DbConnection<'_>,
    _settings: Settings
) -> Result<Settings, AppError> {

    use crate::schema::global_settings::dsl::*;
    let updated = diesel::update(global_settings.find(1))
        .set(&_settings)
        .get_result::<Settings>(conn)
        .await?;

    Ok(updated)
}

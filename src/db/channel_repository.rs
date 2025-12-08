use diesel::{QueryDsl, OptionalExtension};
use diesel_async::RunQueryDsl;
use crate::db::connection::DbConnection;
use crate::errors::AppError;
use crate::models::{Channel};

pub async fn get_channel_by_id(
    conn: &mut DbConnection<'_>,
    _id: &str,
) -> Result<Option<Channel>, AppError> {
    use crate::schema::channels::dsl::*;
    let res = channels
        .find(_id)
        .first::<Channel>(conn)
        .await
        .optional()?;
    Ok(res)
}

// pub async fn get_all_channels(
//     conn: &mut DbConnection<'_>,
// ) -> Result<Vec<Channel>, AppError> {
//     use crate::schema::channels::dsl::*;
//     let res = channels
//         .load::<Channel>(conn)
//         .await?;
//
//     Ok(res)
// }

// pub async fn insert_channel(
//     conn: &mut DbConnection<'_>,
//     new_channel: NewChannel,
// ) -> Result<Channel, AppError> {
//     use crate::schema::channels::dsl::*;
//     let channel = diesel::insert_into(channels)
//         .values(&new_channel)
//         .get_result::<Channel>(conn)
//         .await?;
//
//     Ok(channel)
// }

// pub async fn save_channel(
//     conn: &mut DbConnection<'_>,
//     _channel: Channel,
// ) -> Result<Channel, AppError> {
//
//     use crate::schema::channels::dsl::*;
//     let updated = diesel::update(channels.find(_channel.id.clone()))
//         .set(&_channel)
//         .get_result::<Channel>(conn)
//         .await?;
//
//     Ok(updated)
// }

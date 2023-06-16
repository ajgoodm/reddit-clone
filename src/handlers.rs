use axum::{http::StatusCode, response::Json};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{database::DatabaseConnection, models::User, schema::users, utils::internal_error};

pub async fn list_users(
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let res = users::table
        .select(User::as_select())
        .load(&mut conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(res))
}

use axum::{extract::Path, http::StatusCode, response::Json};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{
    database::DatabaseConnection,
    models::{NewUser, User},
    schema::users,
    utils::internal_error,
};

pub async fn get_user(
    DatabaseConnection(mut conn): DatabaseConnection,
    Path(user_id): Path<i32>,
) -> Result<Json<User>, (StatusCode, String)> {
    let result = users::table
        .select(User::as_select())
        .filter(users::id.eq(user_id))
        .first(&mut conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(result))
}

pub async fn create_user(
    DatabaseConnection(mut conn): DatabaseConnection,
    Json(new_user): Json<NewUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    let result = diesel::insert_into(users::table)
        .values(new_user)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(internal_error)?;
    Ok(Json(result))
}

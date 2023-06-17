use axum::{http::StatusCode, response::Json};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::{
    database::DatabaseConnection,
    models::{NewUser, User},
    schema::users,
    utils::internal_error,
};

pub async fn list_users(
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let result = users::table
        .select(User::as_select())
        .load(&mut conn)
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

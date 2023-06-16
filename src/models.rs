use chrono::prelude::*;
use diesel::prelude::*;

use crate::schema::users;

#[derive(serde::Serialize, Selectable, Queryable)]
pub struct User {
    id: i32,
    name: String,
    created_at: DateTime<Utc>,
}

use crate::schema::users;
use diesel::{Identifiable, Queryable};
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub sir_name: Option<String>,
}

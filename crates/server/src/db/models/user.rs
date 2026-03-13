use crate::db::schema::users;
use chrono::{NaiveDateTime, Utc};

use diesel::prelude::*;

use uuid::Uuid;

#[derive(Identifiable, Queryable, Selectable, Insertable)]
#[diesel(table_name = users)]
#[diesel(treat_none_as_null = true)]
#[diesel(primary_key(uuid))]
pub struct User {
    pub uuid: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

    pub email: String,

    pub auth_key_hash: Vec<u8>,
    pub salt: Vec<u8>,
}

impl User {
    pub fn new(email: &str) -> Self {
        let now = Utc::now().naive_utc();
        let email = email.to_lowercase();

        Self {
            uuid: Uuid::new_v4().to_string(),
            created_at: now,
            updated_at: now,
            email,
            auth_key_hash: Vec::new(),
            salt: Vec::new(),
        }
    }
}

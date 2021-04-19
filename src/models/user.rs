use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
    pub id: u64,
}

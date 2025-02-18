use sqlx::FromRow;

#[derive(Clone, FromRow, Debug)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub done: bool,
}

use serde::Deserialize;
use sqlx::FromRow;

use crate::presentation::components::TodoTemplate;

pub trait Mappable<T> {
    fn map_to(self) -> T;
}

pub fn convert_list<T1, T2>(items: Vec<T1>) -> Vec<T2>
where
    T1: Mappable<T2>,
{
    items.into_iter().map(|item| item.map_to()).collect()
}

#[derive(Clone, FromRow, Debug, Deserialize)]
pub struct TodoCreateForm {
    pub title: String,
}

#[derive(Clone, FromRow, Debug)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub done: bool,
}

impl Mappable<TodoTemplate> for Todo {
    fn map_to(self) -> TodoTemplate {
        TodoTemplate {
            id: self.id,
            title: self.title,
            done: self.done,
        }
    }
}

#[derive(Clone, FromRow, Debug)]
pub struct EntityTodo {
    pub id: i64,
    pub title: String,
    pub done: bool,
}

impl Mappable<Todo> for EntityTodo {
    fn map_to(self) -> Todo {
        Todo {
            id: self.id,
            title: self.title,
            done: self.done,
        }
    }
}

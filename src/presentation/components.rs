use askama::Template;
use axum::{
    body::Body,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

#[derive(Template)]
#[template(path = "pages/Hello.html")]
pub struct HelloTemplate;

// Wrapper around Askama HTML templates to allow them to be served by axum.
pub struct HtmlTemplate<T>(pub T);

/// Allows us to convert Askama HTML templates into valid HTML for axum to serve in the response.
impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response<Body> {
        // Attempt to render the template with askama
        match self.0.render() {
            // If we're able to successfully parse and aggregate the template, serve it
            Ok(html) => Html(html).into_response(),
            // If we're not, return an error or some bit of fallback HTML
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

#[derive(Template)]
#[template(path = "pages/Index.html")]
pub struct LandingPage {
    pub todo_list: TodoList,
}

#[derive(Template)]
#[template(path = "components/TodoList.html")]
pub struct TodoList {
    pub todos: Vec<Todo>,
}

#[derive(Template)]
#[template(path = "components/Todo.html")]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub done: bool,
}

// #[derive(Template)]
// #[template(path = "components/todo.html")]
// pub struct TodoTemplate {
//     pub id: i32,
//     pub title: String,
//     pub completed: bool,
// }

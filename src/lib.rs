pub mod path;

use std::sync::Arc;

use axum::Extension;
use maud::{html, Markup, DOCTYPE};
use serde::Deserialize;
use tokio::sync::Mutex;

#[derive(Default)]
pub struct TodoData {
    pub todos: Vec<String>,
}

pub(crate) fn main_template(body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            body {
                script src="https://unpkg.com/htmx.org@1.9.4" {}
                div hx-boost="true" {
                    (body)
                }
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct TodoForm {
    pub todo_data: String,
}

pub type TodoDB = Extension<Arc<Mutex<TodoData>>>;

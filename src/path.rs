use axum::Form;
use maud::{html, Markup};

use crate::{main_template, TodoDB, TodoForm};

pub async fn index(body: TodoDB) -> Markup {
    let todos = body.0.lock().await;

    let body = html! {
        h1 {
            "Todo App"
        }
        br;
        form hx-post="/add_todo" hx-target="#todo-list" {
            input type="text" id="todo_data" name="todo_data"  { }
            nbsp;
            button type="submit" { ("Add") }
        }
        br;
        ol #todo-list {
            (create_list(&todos.todos))
        }

    };

    main_template(body)
}

pub async fn add_todo(body: TodoDB, form: Form<TodoForm>) -> Markup {
    let arc = body.0.clone();
    let mut todos = arc.lock().await;

    todos.todos.push(form.todo_data.clone());

    create_list(&todos.todos)
}

fn create_list(list: &Vec<String>) -> Markup {
    html! {
        @for todo in list {
            li { (todo) }
        }
    }
}

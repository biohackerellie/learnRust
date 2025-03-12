use askama::Template;
use axum::{
    Router,
    extract::Form,
    http::StatusCode,
    response::Html,
    routing::{get, post},
};
use lazy_static::lazy_static;
use std::sync::Mutex;

// Define a global shared state for storing tasks
lazy_static! {
    static ref TASKS: Mutex<Vec<String>> = Mutex::new(vec![]);
}

// Define the HTML template using Askama
#[derive(Template)]
#[template(inline = r#"
        <h1>Todo List</h1>
        <form action="/add" method="post">
        <input type="text" name="task"/>
        <input type="submit" value="Add Task"/>
        </form>
        <ul>
        {% for task in tasks %}
        <li>{{ task }}</li>
        {% endfor %}
        </ul>
    "#)]
struct TodoListTemplate<'a> {
    tasks: &'a Vec<String>,
}

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(show_tasks))
        .route("/add", post(add_task));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Handler to display tasks
async fn show_tasks() -> Html<String> {
    let tasks = TASKS.lock().unwrap();
    let template = TodoListTemplate { tasks: &tasks };
    Html(template.render().unwrap())
}

// Handler to add a new task
async fn add_task(Form(input): Form<AddTask>) -> StatusCode {
    let mut tasks = TASKS.lock().unwrap();
    tasks.push(input.task);
    StatusCode::SEE_OTHER
}

// Structure to receive form data
#[derive(serde::Deserialize)]
struct AddTask {
    task: String,
}


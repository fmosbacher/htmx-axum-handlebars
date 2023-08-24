use axum::{extract::State, response::Html, routing::get, Router, Server};
use handlebars::Handlebars;
use serde::Serialize;
use std::net::SocketAddr;
use tower_http::services::{ServeDir, ServeFile};

#[derive(Clone)]
struct AppState<'a> {
    handlebars: Handlebars<'a>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        handlebars: Handlebars::new(),
    };

    let app = Router::new()
        .route_service("/", ServeFile::new("index.html"))
        .route("/hello", get(hello))
        .nest_service("/assets", ServeDir::new("assets"))
        .nest_service("/dist", ServeDir::new("dist"))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

#[derive(Serialize)]
struct TemplateData {
    some_number: i32,
}

async fn hello(State(state): State<AppState<'_>>) -> Html<String> {
    let template = include_str!("../templates/hello.hbs");
    let data = TemplateData { some_number: 42 };
    Html(state.handlebars.render_template(template, &data).unwrap())
}

use axum::{extract::State, routing::get, serve, Json, Router};
use mongodb::{options::ClientOptions, Client};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    title: String,
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();
    let client = Client::with_options(client_options).unwrap();

    let app = Router::new()
        .route("/api/todos", get(get_todos).post(create_todo))
        .with_state(client)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let tcp = TcpListener::bind(addr).await.unwrap();

    serve(tcp, app.into_make_service()).await.unwrap();

    Ok(())
}

async fn create_todo(State(client): State<Client>, Json(new_todo): Json<Todo>) -> Json<Todo> {
    let db = client.database("todo_db");
    let collection = db.collection::<Todo>("todos");

    let mut todo = new_todo;
    todo.id = Some(mongodb::bson::oid::ObjectId::new().to_hex());

    collection.insert_one(&todo, None).await.unwrap();

    Json(todo)
}

async fn get_todos(State(client): State<Client>) -> Json<Vec<Todo>> {
    let db = client.database("todo_db");
    let collection = db.collection::<Todo>("todos");

    let mut cursor = collection.find(None, None).await.unwrap();
    let mut todos = Vec::new();

    while cursor.advance().await.unwrap() {
        let x = cursor.deserialize_current().unwrap();
        todos.push(x);
    }

    Json(todos)
}

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Form, Router,
};
use object::{Bookmark, NewBookmark};
use once_cell::sync::Lazy;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
use templates::{BookmarkTemplate, BookmarksTemplate, IndexTemplate};

mod object;
mod templates;

async fn index() -> impl IntoResponse {
    IndexTemplate
}

async fn fetch_bookmarks() -> impl IntoResponse {
    let bookmarks: Vec<Bookmark> = DB.select("bookmark").await.unwrap();
    BookmarksTemplate { bookmarks }
}

async fn create_bookmark(Form(form): Form<NewBookmark>) -> impl IntoResponse {
    let created: Vec<Bookmark> = DB
        .create("bookmark")
        .content(NewBookmark {
            name: form.name,
            url: form.url,
        })
        .await
        .unwrap();

    BookmarkTemplate {
        bookmark: created.first().unwrap().clone(),
    }
}

async fn delete_bookmark(Path(id): Path<String>) -> impl IntoResponse {
    let _: Option<Bookmark> = DB.delete(("bookmark", id)).await.unwrap();

    StatusCode::OK
}

static DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    DB.connect::<Ws>("127.0.0.1:8000").await?;

    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    DB.use_ns("bookman").use_db("test").await?;

    let app = Router::new()
        .route("/", get(index))
        .route("/bookmarks", get(fetch_bookmarks))
        .route("/bookmark", post(create_bookmark))
        .route("/bookmark/:id", delete(delete_bookmark));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

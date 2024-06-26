use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Form, Router,
};
use object::{Bookmark, NewBookmark, NewBookmarkForm, SearchForm, Tags};
use once_cell::sync::Lazy;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};
use templates::{BookmarkEditTemplate, BookmarkTemplate, BookmarksTemplate, IndexTemplate};

mod object;
mod templates;

async fn index() -> impl IntoResponse {
    IndexTemplate
}

async fn fetch_bookmarks() -> impl IntoResponse {
    let bookmarks: Vec<Bookmark> = DB.select("bookmark").await.unwrap();

    BookmarksTemplate { bookmarks }
}

async fn search_bookmarks(Form(form): Form<SearchForm>) -> impl IntoResponse {
    let sql = format!(
        "SELECT * FROM type::table($table) WHERE (name CONTAINS '{}') OR (url CONTAINS '{}') OR (tags CONTAINS '{}')",
        form.content, form.content, form.content
    );
    let mut result = DB.query(sql).bind(("table", "bookmark")).await.unwrap();
    let bookmarks: Vec<Bookmark> = result.take(0).unwrap();

    BookmarksTemplate { bookmarks }
}

async fn fetch_bookmark(Path(id): Path<String>) -> impl IntoResponse {
    let bookmark: Option<Bookmark> = DB.select(("bookmark", id)).await.unwrap();

    BookmarkTemplate {
        bookmark: bookmark.unwrap(),
    }
}

async fn create_bookmark(Form(form): Form<NewBookmarkForm>) -> impl IntoResponse {
    let tags = Tags(
        form.tags
            .split(',')
            .map(|tag| tag.trim().to_string())
            .collect(),
    );
    let created: Vec<Bookmark> = DB
        .create("bookmark")
        .content(NewBookmark {
            name: form.name,
            url: form.url,
            tags,
        })
        .await
        .unwrap();

    BookmarkTemplate {
        bookmark: created.first().unwrap().clone(),
    }
}

async fn edit_bookmark(Path(id): Path<String>) -> impl IntoResponse {
    let bookmark: Option<Bookmark> = DB.select(("bookmark", id)).await.unwrap();

    BookmarkEditTemplate {
        bookmark: bookmark.unwrap(),
    }
}

async fn update_bookmark(
    Path(id): Path<String>,
    Form(form): Form<NewBookmarkForm>,
) -> impl IntoResponse {
    let tags = Tags(
        form.tags
            .split(',')
            .map(|tag| tag.trim().to_string())
            .collect(),
    );
    let updated: Option<Bookmark> = DB
        .update(("bookmark", id))
        .content(NewBookmark {
            name: form.name,
            url: form.url,
            tags,
        })
        .await
        .unwrap();

    BookmarkTemplate {
        bookmark: updated.unwrap(),
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
        .route("/bookmarks/search", post(search_bookmarks))
        .route("/bookmark", post(create_bookmark))
        .route(
            "/bookmark/:id",
            get(fetch_bookmark)
                .put(update_bookmark)
                .delete(delete_bookmark),
        )
        .route("/bookmark/:id/edit", get(edit_bookmark));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

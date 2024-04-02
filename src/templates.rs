use askama::Template;

use crate::object::Bookmark;

#[derive(Template)]
#[template(path = "bookmarks.html")]
pub struct BookmarksTemplate {
    pub bookmarks: Vec<Bookmark>,
}

#[derive(Template)]
#[template(path = "bookmark.html")]
pub struct BookmarkTemplate {
    pub bookmark: Bookmark,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

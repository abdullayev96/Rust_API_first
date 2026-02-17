use loco_rs::prelude::*;

use crate::models::_entities::books;

/// Render a list view of `books`.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<books::Model>) -> Result<Response> {
    format::render().view(v, "book/list.html", data!({"items": items}))
}

/// Render a single `book` view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &books::Model) -> Result<Response> {
    format::render().view(v, "book/show.html", data!({"item": item}))
}

/// Render a `book` create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "book/create.html", data!({}))
}

/// Render a `book` edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &books::Model) -> Result<Response> {
    format::render().view(v, "book/edit.html", data!({"item": item}))
}

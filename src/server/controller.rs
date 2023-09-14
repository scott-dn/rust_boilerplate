use std::sync::Arc;

use axum::{
    extract::State,
    response::{IntoResponse, Json, Response},
};

use crate::{server::Server, services::book::IBookService};

pub async fn get_books<T>(State(state): State<Arc<Server<T>>>) -> Response
where
    T: IBookService + Sync + Send + 'static,
{
    let books = state.services.book.get_books();
    Json(books).into_response()
}

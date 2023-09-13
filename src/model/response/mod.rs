pub mod book;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response<T> {
    pub data: T,
}

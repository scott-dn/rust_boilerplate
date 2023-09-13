use crate::model::response::{book::Book, Response};

pub trait IBookService {
    fn get_books(&self) -> Response<Book>;
}

#[derive(Clone)]
pub struct BookService {}

impl BookService {
    pub fn new() -> Self {
        Self {}
    }
}

impl IBookService for BookService {
    fn get_books(&self) -> Response<Book> {
        Response { data: Book {} }
    }
}

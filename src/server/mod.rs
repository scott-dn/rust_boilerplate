mod controller;

use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use axum::{routing::get, Router};
use controller::get_books;
use tracing::info;

use crate::{config::Config, services::book::IBookService};

#[derive(Clone)]
pub struct Server<T: IBookService> {
    config: Config,
    services: Services<T>,
}

#[derive(Clone)]
struct Services<T: IBookService> {
    book: T,
}

impl<T> Server<T>
where
    T: IBookService + Clone + Sync + Send + 'static,
{
    pub fn new(config: Config, book_service: T) -> Self {
        Self {
            config,
            services: Services { book: book_service },
        }
    }

    pub async fn start(&self) -> Result<()> {
        let app = Router::new()
            .route("/books", get(get_books::<T>))
            .with_state(Arc::new((*self).clone()));
        let http_address = SocketAddr::from(([0, 0, 0, 0], self.config.server.http_port));
        info!("listening on {}", http_address);
        axum::Server::bind(&http_address)
            .serve(app.into_make_service())
            .await?;
        Ok(())
    }
}

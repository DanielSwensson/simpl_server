mod response;
pub use response::Response;

mod app;
pub use app::{read_static_file, App, Route, RouteMatcher, Routes};

mod service;
pub use service::Service;

mod threadpool;
use threadpool::ThreadPool;

use std::fmt;
use std::net::TcpListener;

mod request;
pub use request::{QueryParams, Request};

pub enum HTTPStatus {
    Ok,
    NotFound,
}

impl fmt::Display for HTTPStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HTTPStatus::Ok => write!(f, "HTTP/1.1 200 OK"),
            HTTPStatus::NotFound => write!(f, "HTTP/1.1 404 Not Found"),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum HTTPMethod {
    Get,
    Post,
    Put,
    Delete,
}

pub enum ContentType {
    Json,
    Html,
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ContentType::Json => write!(f, "application/json"),
            ContentType::Html => write!(f, "text/html"),
        }
    }
}

pub struct Server {
    pool: ThreadPool,
    address: String,
}

impl Server {
    pub fn new<F>(factory: F) -> Server
    where
        F: Fn() -> App + Send + Clone + 'static,
    {
        let pool = ThreadPool::new(4, factory);

        Server {
            pool,
            address: "127.0.0.1:7878".to_string(),
        }
    }
    pub fn run(&self) {
        let listener = TcpListener::bind(&self.address).unwrap();
        //let app = Arc::new(app);
        for stream in listener.incoming() {
            let stream = stream.unwrap();

            self.pool.execute(move || stream);
        }
    }
    pub fn bind(mut self, address: &str) -> Self {
        self.address = address.to_string();
        self
    }
}

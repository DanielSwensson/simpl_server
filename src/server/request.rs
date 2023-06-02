use crate::server::HTTPMethod;
use std::{collections::HashMap, path::Path};

pub type QueryParams = HashMap<String, String>;

pub struct Request {
    pub path: Box<Path>,
    pub method: HTTPMethod,
    pub query_params: QueryParams,
}

impl Request {
    pub fn new(path: &str, method: HTTPMethod, query_string: &str) -> Request {
        let query_params = query_string
            .split('&')
            .map(|param| {
                let mut split = param.split('=');
                let key = split.next().unwrap_or("").to_string();
                let value = split.next().unwrap_or("").to_string();
                (key, value)
            })
            .collect::<QueryParams>();

        let path = Path::new(path);
        Request {
            path: Into::into(path),
            method,
            query_params,
        }
    }
}

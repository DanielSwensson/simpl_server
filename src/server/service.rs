use std::path::Path;

use crate::server::{Route, Routes};

#[derive(Clone)]
pub struct Service {
    pub routes: Routes,
    pub namespace: String,
}

impl Service {
    pub fn new(routes: Routes, namespace: &str) -> Service {
        let routes = routes
            .iter()
            .map(|route| {
                let path = format!("{}{}", namespace, route.path.to_str().unwrap());
                let path = Path::new(&path);
                Route {
                    path: Into::into(path),
                    method: route.method.clone(),
                    handler: route.handler,
                }
            })
            .collect();

        Service {
            routes,
            namespace: namespace.to_string(),
        }
    }
}

use crate::server::{Route, Routes};

#[derive(Clone)]
pub struct Service {
    pub routes: Routes,
    pub namespace: String,
}

impl Service {
    pub fn new(routes: Routes, namespace: String) -> Service {
        let routes = routes
            .iter()
            .map(|route| {
                let path = format!("{}{}", namespace, route.path);
                Route {
                    path,
                    verb: route.verb.clone(),
                    handler: route.handler,
                }
            })
            .collect();

        Service { routes, namespace }
    }
}

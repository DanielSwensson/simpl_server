use crate::server::Request;
use crate::server::{HTTPMethod, HTTPStatus, Response, Service};
use std::io::{prelude::*, BufReader};
use std::path::Path;
use std::{fs, net::TcpStream, path};

pub trait RouteMatcher {
    fn find_route(&self, request: &Request) -> Option<&Route>;
}

pub type Routes = Vec<Route>;

impl RouteMatcher for Routes {
    fn find_route(&self, request: &Request) -> Option<&Route> {
        self.iter()
            .find(|route| route.path == request.path && route.method == request.method)
    }
}

pub struct App {
    pub routes: Vec<Route>,
    pub serivces: Vec<Service>,
}

#[derive(Clone)]
pub struct Route {
    pub path: Box<Path>,
    pub handler: fn(Request) -> Response,
    pub method: HTTPMethod,
}

impl Route {
    pub fn new(path: &str, handler: fn(Request) -> Response, method: HTTPMethod) -> Route {
        let path = Path::new(path);

        Route {
            path: Into::into(path),
            handler,
            method,
        }
    }
}

impl App {
    pub fn new() -> App {
        println!("new app");
        App {
            routes: vec![],
            serivces: vec![],
        }
    }
    pub fn route(mut self, route: Route) -> App {
        self.routes.push(route);
        self
    }
    pub fn service(mut self, service: Service) -> App {
        self.serivces.push(service);
        self
    }
    pub fn call(&self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&stream);

        let request_line = buf_reader.lines().next().unwrap().unwrap();

        let method = request_line.split_whitespace().next().unwrap();
        let method = get_method_from_string(method);

        let path = request_line.split_whitespace().nth(1).unwrap().to_string();

        let (path, query_string) = match path.split_once('?') {
            Some((path, query_string)) => (path.to_string(), Some(query_string.to_string())),
            None => (path, None),
        };

        let query_string = query_string.unwrap_or_default();

        let response = self.handle_request(Request::new(&path, method, &query_string));

        match stream.write_all(response.format().as_bytes()) {
            Ok(_) => {}
            Err(e) => println!("Error writing to stream: {}", e),
        }
    }

    fn handle_request(&self, request: Request) -> Response {
        let route = self.find_route(&request);

        route.map_or_else(
            || Response {
                status: HTTPStatus::NotFound,
                body: read_static_file("404"),
                content_type: None,
            },
            |route| (route.handler)(request),
        )
    }
}

impl RouteMatcher for App {
    fn find_route(&self, request: &Request) -> Option<&Route> {
        let route = self
            .serivces
            .iter()
            .find(|&service| request.path.starts_with(&service.namespace))
            .and_then(|service| service.routes.find_route(request));

        if route.is_some() {
            return route;
        }
        self.routes.find_route(request)
    }
}

pub fn read_static_file(file_name: &str) -> Option<String> {
    let path = format!("static/{}.html", file_name);
    let exists = path::Path::new(&path).exists();
    if exists {
        Some(fs::read_to_string(path).unwrap())
    } else {
        None
    }
}

fn get_method_from_string(verb: &str) -> HTTPMethod {
    match verb {
        "GET" => HTTPMethod::Get,
        "POST" => HTTPMethod::Post,
        "PUT" => HTTPMethod::Put,
        "DELETE" => HTTPMethod::Delete,
        _ => panic!("Unknown verb"),
    }
}

use crate::server::{HTTPStatus, HTTPVerb, Response, Service};
use std::io::{prelude::*, BufReader};
use std::{fs, net::TcpStream, path};

pub trait RouteMatcher {
    fn find_route(&self, request: &Request) -> Option<&Route>;
}

pub type Routes = Vec<Route>;

impl RouteMatcher for Routes {
    fn find_route(&self, request: &Request) -> Option<&Route> {
        self.iter()
            .find(|route| route.path == request.path && route.verb == request.verb)
    }
}

pub struct App {
    pub routes: Vec<Route>,
    pub serivces: Vec<Service>,
}

#[derive(Clone)]
pub struct Route {
    pub path: String,
    pub handler: fn() -> Response,
    pub verb: HTTPVerb,
}

pub struct Request {
    pub path: String,
    pub verb: HTTPVerb,
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

        let verb = request_line.split_whitespace().next().unwrap();
        let verb = get_verb_from_string(verb);

        let path = request_line.split_whitespace().nth(1).unwrap().to_string();

        let response = self.handle_request(Request { path, verb });
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
            |route| (route.handler)(),
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

fn get_verb_from_string(verb: &str) -> HTTPVerb {
    match verb {
        "GET" => HTTPVerb::Get,
        "POST" => HTTPVerb::Post,
        "PUT" => HTTPVerb::Put,
        "DELETE" => HTTPVerb::Delete,
        _ => panic!("Unknown verb"),
    }
}

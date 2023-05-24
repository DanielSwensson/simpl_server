mod server;

fn main() {
    server::Server::new(|| {
        server::App::new()
            .route(server::Route {
                path: String::from("/"),
                handler: || server::Response {
                    status: server::HTTPStatus::Ok,
                    body: server::read_static_file("index"),
                    content_type: Some(server::ContentType::Html),
                },
                verb: server::HTTPVerb::Get,
            })
            .service(server::Service::new(
                vec![server::Route {
                    path: String::from("/hello"),
                    handler: || server::Response {
                        status: server::HTTPStatus::Ok,
                        body: Some(String::from("Hello!!")),
                        content_type: Some(server::ContentType::Html),
                    },
                    verb: server::HTTPVerb::Get,
                }],
                "/api".to_string(),
            ))
    })
    .bind("127.0.0.1:8080")
    .run();

    println!("Shutting down.");
}

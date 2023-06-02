mod server;

fn main() {
    server::Server::new(|| {
        server::App::new()
            .route(server::Route::new(
                "/",
                |_query_params| server::Response {
                    status: server::HTTPStatus::Ok,
                    body: server::read_static_file("index"),
                    content_type: Some(server::ContentType::Html),
                },
                server::HTTPMethod::Get,
            ))
            .service(server::Service::new(
                vec![server::Route::new(
                    "/hello",
                    |request| {
                        let name = request.query_params.get("name");

                        let name = match name {
                            Some(name) => name,
                            None => "World",
                        };

                        let count = request.query_params.get("count");
                        let count = match count {
                            Some(count) => count,
                            None => "1",
                        };

                        server::Response {
                            status: server::HTTPStatus::Ok,
                            body: Some(format!("Hello, {:} count: {:}", name, count)),
                            content_type: Some(server::ContentType::Html),
                        }
                    },
                    server::HTTPMethod::Get,
                )],
                "/api",
            ))
    })
    .bind("127.0.0.1:8080")
    .run();

    println!("Shutting down.");
}

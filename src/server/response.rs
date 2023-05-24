use crate::server::{ContentType, HTTPStatus};

pub struct Response {
    pub status: HTTPStatus,
    pub body: Option<String>,
    pub content_type: Option<ContentType>,
}

impl Response {
    pub fn format(self) -> String {
        let body = self.body.as_ref().unwrap();

        let length = body.len();
        let status_line = self.status.to_string();

        let content_type = self.content_type.unwrap_or(ContentType::Html).to_string();

        format!(
            "{status_line}\r\nContent-Length: {length} Content-Type={content_type} \r\n\r\n{body}"
        )
    }
}

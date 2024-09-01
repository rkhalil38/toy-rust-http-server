pub struct Route {
    pub path: String,
    pub content_type: String,
    pub payload: String,
}

impl Route {
    pub fn form_response(&self) -> String {
        let mut response = String::new();

        response.push_str("HTTP/1.1 200 OK\r\nContent-Type: ");
        response.push_str(&self.content_type);
        response.push_str("\r\n\r\n");
        response.push_str(&self.payload);

        return response;
    }
}




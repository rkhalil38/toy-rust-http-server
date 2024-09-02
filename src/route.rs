//Representation of Route
pub struct Route {
    //path that server recognizes
    pub path: String,
    //type of content used for headers
    pub content_type: String,
    //payload that is delivered after hitting endpoint
    pub payload: String,
}

impl Route {
    //Constructs HTTP response that can be served by RouteRegistry
    pub fn form_response(&self) -> String {
        let mut response = String::new();

        response.push_str("HTTP/1.1 200 OK\r\nContent-Type: ");
        response.push_str(&self.content_type);
        response.push_str("\r\n\r\n");
        response.push_str(&self.payload);

        return response;
    }
}




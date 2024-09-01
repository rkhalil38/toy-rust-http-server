use std::collections::HashMap;
use crate::{route::Route, util::stringify_file};
pub struct RouteRegistry {
    routes: HashMap<String, String>
}

impl RouteRegistry {
    pub fn new() -> RouteRegistry {
        RouteRegistry {
            routes: HashMap::new(),
        }
    }

    pub fn create_route(&mut self, path: &str, content: &str, payload_source: &str) {
        let content_type = match content {
            "json" => "application/json\r\n",
            "html" => "text/html\r\n",
            "text" => "text/plain\r\n",
            "css" => "text/css\r\n",
            _ => panic!("CONTENT TYPE NOT SUPPORTED")
        };
    
        let payload = match content {
            "json" => payload_source,
            "html" => &stringify_file(payload_source),
            "text" => payload_source,
            "css" => &stringify_file(payload_source),
            _ => panic!("CONTENT TYPE NOT SUPPORTED")
        };
    
        let route = Route {
            path: String::from(path),
            content_type: String::from(content_type),
            payload: String::from(payload),
        };

        self.routes.insert(String::from(path), route.form_response());
        
    }

    pub fn serve_route(&self, request: &str) -> String {
        return match self.routes.get(request) {
            Some(x) => String::from(x),
            None => String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n")
        };
    }

}
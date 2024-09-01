# toy-rust-http-server

This is a simple rust HTTP server that is intended for learning and interactive use.
None of this code is remotely ready for any kind of use in production, so please, don't make it what it's not.

## Getting Started

### Clone the Repo
```
git clone https://github.com/rkhalil38/toy-rust-http-server.git
```

### Change into the Working Directory
```
cd toy-rust-http-server
```
### Install Dependencies
```
cargo build
```
### Run the Server
```
cargo run main.rs
```

## Using the Server

### Creating a Route Registry
To create a route, we first need a `RouteRegistry` object. This object needs to be mutable because we will be adding routes to it.
```rust
#[tokio::main]
async fn main() {
  let mut route_registry = RouteRegistry::new();
}
```
### Creating a Route
Using the `RouteRegistry` object we just created, we can create routes.
```rust
#[tokio::main]
async fn main() {
    let mut route_registry = RouteRegistry::new();
    route_registry.create_route("/retrievehtml", "html", "./main.html");
}
```
Here, we call the `create_route` method and pass in our desired path, content type, and the path of the content source.

### Hitting the Endpoint
Now, if we run the server and hit the endpoint, the HTML file we pointed to will be returned.
```
curl http://localhost:4221/retrievehtml
```
If we enter the URL into the browser, the HTML will be fully rendered.
### Other Content Types
Currently, the server supports text, HTML, CSS, and JSON.
```rust
route_registry.create_route("/stylesheet.css", "css", "./stylesheet.css");
route_registry.create_route("/retrievetext", "text", "this is a text message");
```
JSON requires a little extra effort.
```rust
let json_example = r#"
  {
    "student": {
                  "romulus": "GA TECH"
               },
  }
"#;

route_registry.create_route("/retrievejson", "json", json_example);
```

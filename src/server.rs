use std::{net::SocketAddr, sync::Arc};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

pub struct ServerConnection {
    connection: TcpStream,
}

pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    PATCH,
    TRACE,
    CONNECT,
}

pub struct Request {
    method: HttpMethod,
    version: String,
    host: SocketAddr,
    accept: Doctype,
}

pub struct Response {
    version: String,
    doctype: Doctype,
    length: i32,
    content: String,
}

pub enum Doctype {
    Html,
    Json,
}

impl ServerConnection {
    pub async fn empty() -> Result<Self, Box<dyn std::error::Error>> {
        // returns a handle to localhost
        let connection;
        match TcpStream::connect("127.0.0.1::8080").await {
            Ok(stream) => connection = stream,
            Err(e) => {
                return Err(Box::new(e));
            }
        }
        Ok(Self { connection })
    }
    pub async fn construct() -> Self {
        todo!()
    }
    pub async fn handle_request(request: &str) {}
}

impl HttpMethod {
    pub fn from_str(method: &str) -> Option<HttpMethod> {
        match method.to_uppercase().as_str() {
            "GET" => Some(HttpMethod::GET),
            "POST" => Some(HttpMethod::POST),
            "PUT" => Some(HttpMethod::PUT),
            "DELETE" => Some(HttpMethod::DELETE),
            "HEAD" => Some(HttpMethod::HEAD),
            "OPTIONS" => Some(HttpMethod::OPTIONS),
            "PATCH" => Some(HttpMethod::PATCH),
            "TRACE" => Some(HttpMethod::TRACE),
            "CONNECT" => Some(HttpMethod::CONNECT),
            _ => None,
        }
    }
}

impl Request {
    pub fn parse(request: String) {
        // parses http request, returns user ID specified
        let request = request.lines();
    }
}

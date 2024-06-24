use std::error::Error;
use std::fmt;
use std::{
    net::{SocketAddr, ToSocketAddrs},
    str::FromStr,
    sync::Arc,
};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

#[derive(Debug)]
pub struct ServerConnection {
    connection: TcpStream,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Request {
    method: Option<HttpMethod>,
    version: Option<String>,
    host: Option<SocketAddr>,
    accept: Option<Doctype>,
    resource: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Response {
    version: String,
    doctype: Doctype,
    length: i32,
    content: String,
}

#[derive(Debug, Clone)]
pub enum Doctype {
    Html,
    Json,
}

impl ServerConnection {
    pub async fn empty() -> Result<Self, Box<dyn Error>> {
        let connection = TcpStream::connect("127.0.0.1:8080").await?;
        Ok(Self { connection })
    }

    pub async fn construct() -> Self {
        todo!()
    }

    pub async fn handle_request(request: &str) {}
}

impl FromStr for HttpMethod {
    type Err = &'static str;

    fn from_str(method: &str) -> Result<Self, Self::Err> {
        match method.to_uppercase().as_str() {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            "HEAD" => Ok(HttpMethod::HEAD),
            "OPTIONS" => Ok(HttpMethod::OPTIONS),
            "PATCH" => Ok(HttpMethod::PATCH),
            "TRACE" => Ok(HttpMethod::TRACE),
            "CONNECT" => Ok(HttpMethod::CONNECT),
            _ => Err("Invalid HTTP method"),
        }
    }
}

// Custom error type for `FromStr` implementations
#[derive(Debug)]
pub struct ParseError(&'static str);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for ParseError {}

impl FromStr for Doctype {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            s if s.contains("application/json") => Ok(Doctype::Json),
            s if s.contains("text/html") => Ok(Doctype::Html),
            _ => Err(ParseError("Invalid Doctype")),
        }
    }
}

impl Request {
    pub fn parse(request: String) -> Result<Self, Box<dyn Error>> {
        let lines: Vec<&str> = request.lines().collect();
        let first_line = lines.get(0).ok_or("Request is empty")?;
        let parts: Vec<&str> = first_line.split_whitespace().collect();

        if parts.len() < 3 {
            return Err("Invalid request line".into());
        }

        let method = HttpMethod::from_str(parts[0])?;
        let resource = if parts[1] == "/" {
            println!("No resource requested!");
            None
        } else {
            Some(parts[1].to_string())
        };
        let version = parts[2]
            .split_once('/')
            .ok_or("Invalid HTTP version")?
            .1
            .to_string();

        let host_line = lines.get(1).ok_or("Host line missing")?;
        let host_str = host_line.split_once(' ').ok_or("Invalid host line")?.1;
        let host = host_str
            .to_socket_addrs()?
            .next()
            .ok_or("Invalid host address")?;

        let mut accept = Doctype::Html;
        for line in &lines[1..] {
            if line.starts_with("Accept:") {
                accept = Doctype::from_str(
                    line.split_once(':')
                        .ok_or("Invalid Accept header")?
                        .1
                        .trim(),
                )
                .unwrap_or(Doctype::Html);
            }
        }

        println!(
            "METHOD: {:?}\nVERSION: {}\nHOST: {:?}\nDOCTYPE: {:?}\nRESOURCE: {:?}",
            method, version, host, accept, resource
        );

        Ok(Self {
            method: Some(method),
            version: Some(version),
            host: Some(host),
            accept: Some(accept),
            resource,
        })
    }

    pub fn shart(request: String) -> Result<Self, Box<dyn std::error::Error>> {
        // ignore this whole function
        // this was supposed to be a better parse method but it doesnt work for some reaoson
        let mut method: Option<HttpMethod> = None;
        let mut resource: Option<String> = None;
        let mut version: Option<String> = None;
        let mut host: Option<SocketAddr> = None;
        let mut accept: Option<Doctype> = None;

        // why did i do this
        let mut start_line_parsed = false;

        for line in request.lines() {
            // this shit should never panic on split but we do this anyway
            if !line.is_empty() {
                let parts: Vec<&str> = line.split_whitespace().collect();

                // Only parse the start line once
                if !start_line_parsed && parts.len() == 3 && parts[2].starts_with("HTTP/") {
                    method = Some(HttpMethod::from_str(parts[0]).unwrap_or(HttpMethod::GET));
                    resource = Some(parts[1].to_string());
                    version = Some(parts[2][5..].to_string()); // Extract version after "HTTP/"
                    start_line_parsed = true; // start line now parsed
                }
                // Host header
                else if line.starts_with("Host:") {
                    let host_str = line
                        .split_once(": ")
                        .map(|(_, v)| v)
                        .unwrap_or("0.0.0.0:8080");
                    host = Some(host_str.to_socket_addrs()?.next().ok_or("Invalid host")?);
                }
                // Accept header
                else if line.starts_with("Accept:") {
                    let doctype_str = line.split_once(": ").map(|(_, v)| v).unwrap_or("text/html"); // Get the first value only
                    accept = Some(Doctype::from_str(doctype_str).unwrap_or(Doctype::Html));
                }
            }
        }

        if let (Some(method), Some(resource), Some(version), Some(host)) =
            (method, resource, version, host)
        {
            println!(
                "METHOD: {:?}\nRESOURCE: {:?}\nVERSION: {:?}\nHOST: {:?}, DOCTYPE: {:?}\n",
                method, resource, version, host, accept
            );

            Ok(Self {
                method: Some(method),
                version: Some(version),
                host: Some(host),
                accept,
                resource: Some(resource),
            })
        } else {
            Err("Missing required fields in HTTP request".into())
        }
    }
}

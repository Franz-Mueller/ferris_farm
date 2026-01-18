use std::{
    fmt::Error,
    io::{BufReader, prelude::*},
    net::TcpStream,
};

enum HttpMethod {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

pub struct HttpRequest {
    method: HttpMethod,
    target: String,
    version: String,
    body: String,
}

impl HttpRequest {
    pub fn new(buf_reader: BufReader<&TcpStream>) -> Result<HttpRequest, &'static str> {
        let request_line = buf_reader.lines().next().unwrap().unwrap();
        let request_line: Vec<&str> = request_line.split(" ").collect();

        let method: HttpMethod = match request_line[0] {
            "GET" => HttpMethod::Get,
            "HEAD" => HttpMethod::Head,
            "POST" => HttpMethod::Post,
            "PUT" => HttpMethod::Put,
            "DELETE" => HttpMethod::Delete,
            "CONNECT" => HttpMethod::Connect,
            "OPTIONS" => HttpMethod::Options,
            "TRACE" => HttpMethod::Trace,
            "PATCH" => HttpMethod::Patch,
            _ => return Err("invalid method"),
        };

        Ok(HttpRequest {
            method: method,
            target: request_line[1].to_string(),
            version: request_line[2].to_string(),
            body: "lol".to_string(),
        })
    }
}

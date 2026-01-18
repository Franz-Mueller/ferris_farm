use std::{
    collections::HashMap,
    io,
    io::{BufReader, prelude::*},
    net::TcpStream,
};

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub target: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl HttpRequest {
    pub fn read_from_stream(stream: &mut TcpStream) -> io::Result<Self> {
        let mut reader = BufReader::new(stream);

        let mut request_line = String::new();
        reader.read_line(&mut request_line)?;
        if request_line.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "empty request line",
            ));
        }
        let request_line = request_line.trim_end_matches(&['\r', '\n'][..]);

        let mut parts = request_line.split_whitespace();
        let method = parts
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "missing method"))?
            .to_string();
        let target = parts
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "missing target"))?
            .to_string();
        let version = parts
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "missing version"))?
            .to_string();

        if parts.next().is_some() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "malformed request line",
            ));
        }

        let mut headers = HashMap::new();
        loop {
            let mut line = String::new();
            reader.read_line(&mut line)?;
            if line.is_empty() {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "eof in headers",
                ));
            }

            if line == "\r\n" {
                break;
            }

            let line = line.trim_end_matches(&['\r', '\n'][..]);

            let (name, value) = line
                .split_once(':')
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "bad header line"))?;

            headers.insert(name.trim().to_ascii_lowercase(), value.trim().to_string());
        }

        let mut body = Vec::new();

        if let Some(cl) = headers.get("content-length") {
            let len: usize = cl
                .parse()
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "bad content-length"))?;

            body.resize(len, 0);
            reader.read_exact(&mut body)?;
        } else if let Some(te) = headers.get("transfer-encoding") {
            if te.to_ascii_lowercase().contains("chunked") {
                return Err(io::Error::new(
                    io::ErrorKind::Unsupported,
                    "chunked transfer-encoding not supported",
                ));
            }
        }

        Ok(HttpRequest {
            method,
            target,
            version,
            headers,
            body,
        })
    }
    pub fn get_body_as_string(&self) -> String {
        String::from_utf8_lossy(&self.body).to_string()
    }
}

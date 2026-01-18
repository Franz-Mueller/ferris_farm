use ferris_farm::server::{requests::HttpRequest, threads::ThreadPool};
use std::net::{TcpListener, TcpStream};

const PAGES: [&str; 2] = ["/api/sensor/hum_temp", "lol"];

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    match HttpRequest::read_from_stream(&mut stream) {
        Ok(req) => {
            pass_request(req);
        }
        Err(e) => {}
    }
}

fn pass_request(req: HttpRequest) {
    for page in PAGES {
        if req.target == page {
            println!("{:#?}", req.get_body_as_string());
            break;
        }
    }
}

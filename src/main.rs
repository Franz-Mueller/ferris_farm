use ferris_farm::http::{
    requests::HttpRequest, sensor_read::read_sensor_message, threads::ThreadPool,
};
use std::net::{TcpListener, TcpStream};

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
            println!("{:#?}", req.get_body_as_string());
        }
        Err(e) => {
            eprintln!("bad request: {e}");
        }
    }
}

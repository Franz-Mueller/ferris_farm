use ferris_farm::{
    sensors::sensor,
    server::{requests::HttpRequest, threads::ThreadPool},
};
use std::{
    io,
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").expect("failed to bind to 0.0.0.0:7878");
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                println!("could not establish connection: {e}");
                continue;
            }
        };

        pool.execute(|| {
            if let Err(e) = handle_connection(stream) {
                println!("Error: {}", e);
            };
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) -> Result<(), io::Error> {
    let req = HttpRequest::read_from_stream(&mut stream)?;
    pass_request(req)?;
    Ok(())
}

fn pass_request(req: HttpRequest) -> Result<(), io::Error> {
    let sensor: sensor::AnySensor = match req.target.as_str() {
        "/api/sensor/temp" => sensor::AnySensor::Temp(sensor::TempSensor {}),
        "/api/sensor/hum" => sensor::AnySensor::Hum(sensor::HumSensor {}),
        "/api/sensor/lux" => sensor::AnySensor::Lux(sensor::LuxSensor {}),
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("unknown sensor endpoint {}", req.target),
            ));
        }
    };

    let body = req.get_body_as_string();
    sensor
        .process(&body)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

    Ok(())
}

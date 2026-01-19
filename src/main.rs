use ferris_farm::{
    sensors::sensor,
    server::{requests::HttpRequest, threads::ThreadPool},
};
use std::{
    fmt::Error,
    net::{TcpListener, TcpStream},
};

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

fn pass_request(req: HttpRequest) -> Result<(), &'static str> {
    println!("{}", req.target.as_str());
    let sensor: sensor::AnySensor = match req.target.as_str() {
        "/api/sensor/temp" => sensor::AnySensor::Temp(sensor::TempSensor {}),
        "/api/sensor/hum" => sensor::AnySensor::Hum(sensor::HumSensor {}),
        "/api/sensor/lux" => sensor::AnySensor::Lux(sensor::LuxSensor {}),
        _ => return Err("unknown sensor endpoint"),
    };

    let body = req.get_body_as_string();
    sensor.process(&body);

    Ok(())
}

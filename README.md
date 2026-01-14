# 🦀 FerrisFarm

FerrisFarm is a personal project to equip an indoor greenhouse with sensors and actuators to monitor and control the environment. The sensors are ESP32-based and measure air temperature, air humidity, light intensity, soil moisture, and soil temperature. The control hub is a Raspberry Pi running an web server using a database to store the sensors data. A second sqlite database will provide storage for settings and other stuff. The Pi provides a frontend for visualizing data and controlling actuators such as an LED grow light and an inline fan plus an API.

This project is a learning exercise and not intended for production use or to replace commercial greenhouse systems. It's primarily a way for me to practice **Rust** and have a bit of fun. Feel free to explore the code or try the setup if you like.

The project is currently under active development and is not yet ready for use.

## Development Principles

This project serves as a learning exercise aimed at improving my proficiency in Rust. To achieve this, I have established the following principles:

I will...

1. Maximize the use of the Rust standard library and minimize reliance on third-party crates.
2. Begin with the multithreaded server implementation from the final chapter of "The Rust Programming Language" book.
3. Embrace rewrites, even if they are challenging or time-consuming.
4. Prioritize learning Rust above all else. Development of ESP32 code and the frontend will be deprioritized. 
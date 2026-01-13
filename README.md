# 🦀 FerrisFarm

FerrisFarm is a personal project to equip an indoor greenhouse with sensors and actuators to monitor and control the environment. The sensors are ESP32-based and measure air temperature, air humidity, light intensity, soil moisture, and soil temperature. The control hub is a Raspberry Pi running an Axum web server and using two databases: one optimized for fast reads/writes of monitoring data, and another for user accounts, settings, and related data. The Pi provides a web-based frontend for visualizing data and controlling actuators such as an LED grow light and an inline fan.

This project is a learning exercise and not intended for production use or to replace commercial greenhouse systems. It's primarily a way for me to practice **Rust** and have a bit of fun. Feel free to explore the code or try the setup if you like.

The project is under active development and not yet ready.

## Setup

**Install sqlx-cli**
```bash
cargo install sqlx-cli --features postgres
```
For later:
```bash
cargo install sqlx-cli --features sqlite
```
**Start PostgreSQL Contaienr**
```bash
docker run -d --name postgres-14 -p 5432:5432 -e POSTGRES_PASSWORD={password} postgres:14
```
**Setup DB**
```bash
sqlx db setup
```
**Migrate**
```bash
sqlx migrate run
```

## Sources

This is my very first real project in Rust which means I will heavily rely on a bunch of resources in order to learn how to do things. I guess in the beginning I will copy a lot of code. Later on I will gain confidence (hopefully) and do the most on my own. The following list will contain my main sources of information on how to do things:

- [realworld-axum-sqlx](https://github.com/launchbadge/realworld-axum-sqlx)
- [measuring humidity and temp with esp32](https://microcontrollerslab.com/esp32-sht31-temperature-humidity-sensor-tutorial/)
- [measuring luix with esp32](https://www.14core.com/wiring-the-tls2591-high-range-lux-light-intensity-ambient-light-sensor-on-microcontroller/)
# 🦀 FerrisFarm

FerrisFarm is a personal project to equip an indoor greenhouse with sensors and actuators to monitor and control the environment. The sensors are ESP32-based and measure air temperature, air humidity, light intensity, soil moisture, and soil temperature. The control hub is a Raspberry Pi running a self made web server (based on the final chapter of "The Rust Programming Language") and using two databases: one optimized for fast reads/writes of monitoring data, and another for user accounts, settings, and related data. The Pi provides a web-based frontend for visualizing data and controlling actuators such as an LED grow light and an inline fan.

This project is a learning exercise and not intended for production use or to replace commercial greenhouse systems. It's primarily a way for me to practice **Rust** and have a bit of fun. Feel free to explore the code or try the setup if you like.

The project is under active development and not yet ready.

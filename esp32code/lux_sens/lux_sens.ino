#include <Arduino.h>
#include <Wire.h>
#include <WiFi.h>
#include <HTTPClient.h>

#include <Adafruit_Sensor.h>
#include "Adafruit_TSL2591.h"

// --- WiFi + endpoint ---
const char WIFI_SSID[]     = "+++++++++++++++";
const char WIFI_PASSWORD[] = "+++++++++++++++";

const char HOST_NAME[] = "http://192.168.178.44:7878/";
const char PATH_NAME[] = "api/sensor/lux";   // <-- adjust on your Rust server

// --- Sensor ---
Adafruit_TSL2591 tsl(2591);

static void wifi_connect() {
  WiFi.begin(WIFI_SSID, WIFI_PASSWORD);
  Serial.print("WiFi");
  while (WiFi.status() != WL_CONNECTED) {
    delay(300);
    Serial.print(".");
  }
  Serial.print("\nIP: ");
  Serial.println(WiFi.localIP());
}

static void tsl_configure() {
  if (!tsl.begin()) {
    Serial.println("TSL2591 not found. Check wiring/I2C address.");
    while (true) delay(1000);
  }

  // Keep it simple. Change if needed.
  tsl.setGain(TSL2591_GAIN_MED);                 // 25x
  tsl.setTiming(TSL2591_INTEGRATIONTIME_100MS);  // fast-ish
}

static String build_payload() {
  // Raw channels (optional but useful)
  uint32_t lum = tsl.getFullLuminosity();
  uint16_t ir   = lum >> 16;
  uint16_t full = lum & 0xFFFF;

  // Lux estimate (main value)
  float lux = tsl.calculateLux(full, ir);

  // Minimal, easy to parse on server side:
  // "lux:123.45, ir:123, full:456"
  String s;
  s.reserve(64);
  s += "lux:";  s += lux;
  s += ", ir:"; s += ir;
  s += ", full:"; s += full;
  return s;
}

static void post_reading(const String& payload) {
  if (WiFi.status() != WL_CONNECTED) {
    wifi_connect();
  }

  HTTPClient http;
  http.begin(String(HOST_NAME) + PATH_NAME);
  http.setTimeout(5000);
  http.addHeader("Content-Type", "application/x-www-form-urlencoded");

  int code = http.POST((uint8_t*)payload.c_str(), payload.length());

  if (code > 0) {
    if (code == HTTP_CODE_OK) {
      Serial.println(http.getString());
    } else {
      Serial.printf("[HTTP] POST code: %d\n", code);
    }
  } else {
    Serial.printf("[HTTP] POST failed: %s\n", http.errorToString(code).c_str());
  }

  http.end();
}

void setup() {
  Serial.begin(9600);

  Wire.begin();          // ESP32 default SDA=21, SCL=22 (matches your note)
  wifi_connect();
  tsl_configure();
}

void loop() {
  String payload = build_payload();
  Serial.println(payload);

  post_reading(payload);
  delay(1000);
}

/*
Wiring (typical ESP32):
VIN -> 3.3V
GND -> GND
SCL -> GPIO 22
SDA -> GPIO 21
*/
// Source: https://microcontrollerslab.com/esp32-sht31-temperature-humidity-sensor-tutorial/
#include <WiFi.h>
#include <HTTPClient.h>
#include <Arduino.h>
#include <Wire.h>
#include "Adafruit_SHT31.h"

Adafruit_SHT31 sht31 = Adafruit_SHT31();

const char WIFI_SSID[] = "+++++++++++++++";
const char WIFI_PASSWORD[] = "+++++++++++++++";
String HOST_NAME   = "http://192.168.178.44:7878/";
String PATH_NAME   = "api/sensor/hum_temp"; 

void setup() {
  Serial.begin(9600);

  WiFi.begin(WIFI_SSID, WIFI_PASSWORD);
  Serial.println("Connecting");
  while(WiFi.status() != WL_CONNECTED) {
    delay(500);
    Serial.print(".");
  }
  Serial.println("");
  Serial.print("Connected to WiFi network with IP Address: ");
  Serial.println(WiFi.localIP());

  if (! sht31.begin(0x44)) {   
    Serial.println("Check circuit. SHT31 not found!");
    while (1) delay(1);
  }
}

void loop() {
  HTTPClient http;

  http.begin(HOST_NAME + PATH_NAME);
  http.addHeader("Content-Type", "application/x-www-form-urlencoded");

  float temp = sht31.readTemperature();
  float hum = sht31.readHumidity();

  String queryString;

  if ((!isnan(temp)) && (!isnan(hum))) {
    queryString = String("temp:") + temp + ", hum:" + hum;
    Serial.println("OK");
  } else {
    queryString = "could not read temp or hum data";
    Serial.println("ERROR");
  }
  http.setTimeout(5000);
  http.addHeader("Content-Type", "application/x-www-form-urlencoded");

  int httpCode = http.POST((uint8_t*)queryString.c_str(), queryString.length());
  if (httpCode > 0) {
    // file found at server
    if (httpCode == HTTP_CODE_OK) {
      String payload = http.getString();
      Serial.println(payload);
    } else {
      // HTTP header has been send and Server response header has been handled
      Serial.printf("[HTTP] POST... code: %d\n", httpCode);
    }
  } else {
    Serial.printf("[HTTP] POST... failed, error: %s\n", http.errorToString(httpCode).c_str());
  }
  http.end();
  delay(1000);
}

// VIN -> 3.3V
// GND -> GND
// SCL -> GPIO 22
// SDA -> GPIO 21
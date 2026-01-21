// Source: https://microcontrollerslab.com/esp32-sht31-temperature-humidity-sensor-tutorial/
#include <WiFi.h>
#include <HTTPClient.h>
#include <Arduino.h>
#include <Wire.h>
#include "Adafruit_SHT31.h"

Adafruit_SHT31 sht31 = Adafruit_SHT31();

const char WIFI_SSID[] = "IamRoutingForMe2,4";
const char WIFI_PASSWORD[] = "Hundeaugen2003";
String HOST_NAME   = "http://192.168.178.44:7878/";
String PATH_NAME_TEMP   = "api/sensor/temp"; 
String PATH_NAME_HUM   = "api/sensor/hum"; 

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
  HTTPClient http_temp;
  HTTPClient http_hum;

  http_temp.begin(HOST_NAME + PATH_NAME_TEMP);
  http_temp.addHeader("Content-Type", "application/x-www-form-urlencoded");

  http_hum.begin(HOST_NAME + PATH_NAME_HUM);
  http_hum.addHeader("Content-Type", "application/x-www-form-urlencoded");

  float temp = sht31.readTemperature();
  float hum = sht31.readHumidity();

  String queryStringTemp;
  String queryStringHum;

  if (!isnan(temp)){
    queryStringTemp = String(temp);
    Serial.println("Temp OK");
  } else {
    queryStringTemp = "NA";
    Serial.println("Temp ERROR");
  }

  if (!isnan(hum)){
    queryStringHum = String(hum);
    Serial.println("Hum OK");
  } else {
    queryStringHum = "NA";
    Serial.println("Hum ERROR");
  }

  http_temp.setTimeout(5000);
  http_temp.addHeader("Content-Type", "application/x-www-form-urlencoded");
  http_temp.POST((uint8_t*)queryStringTemp.c_str(), queryStringTemp.length());

  http_hum.setTimeout(5000);
  http_hum.addHeader("Content-Type", "application/x-www-form-urlencoded");
  http_hum.POST((uint8_t*)queryStringHum.c_str(), queryStringHum.length());
  
  http_temp.end();
  http_hum.end();
  delay(1000);
}

// VIN -> 3.3V
// GND -> GND
// SCL -> GPIO 22
// SDA -> GPIO 21
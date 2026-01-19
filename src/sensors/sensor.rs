pub trait Sensor {
    type Value;
    /// Processes a temperature value from a http request body recieved as string and returns the value ready for db
    fn process_measurement(&self, value: &str) -> Self::Value;
    fn write_to_db(&self, value: &Self::Value) -> Result<(), String>;
}

pub struct TempSensor;
pub enum AnySensor {
    Temp(TempSensor),
    Hum(HumSensor),
    Lux(LuxSensor),
}

impl AnySensor {
    pub fn process(&self, body: &str) {
        match self {
            AnySensor::Temp(s) => {
                let value: f32 = s.process_measurement(body);
                s.write_to_db(&value);
            }
            AnySensor::Hum(s) => {
                let value: f32 = s.process_measurement(body);
                s.write_to_db(&value);
            }
            AnySensor::Lux(s) => {
                let value: (f32, f32) = s.process_measurement(body);
                s.write_to_db(&value);
            }
        }
    }
}
impl Sensor for TempSensor {
    type Value = f32;

    fn process_measurement(&self, value: &str) -> f32 {
        value.parse::<f32>().unwrap()
    }
    fn write_to_db(&self, value: &f32) -> Result<(), String> {
        println!("DB write temp: {value}");
        Ok(())
    }
}

pub struct HumSensor;

impl Sensor for HumSensor {
    type Value = f32;

    fn process_measurement(&self, value: &str) -> f32 {
        value.parse::<f32>().unwrap()
    }
    fn write_to_db(&self, value: &f32) -> Result<(), String> {
        println!("DB write hum: {value}");
        Ok(())
    }
}
pub struct LuxSensor;

impl Sensor for LuxSensor {
    type Value = (f32, f32);

    /// takes lux as &str and returns tuple (f32, f32) conatining lux and ppfd
    fn process_measurement(&self, value: &str) -> (f32, f32) {
        let lux: f32 = value.parse::<f32>().unwrap();
        // TODO should later be changed to user choosable value
        // 0,0185 is estimated value for LED Full-Spectrum
        (lux, lux * 0.0185)
    }
    /// takes tuple (f32, f32) conatining lux and ppfd
    fn write_to_db(&self, value: &(f32, f32)) -> Result<(), String> {
        println!("DB write lux: {} ppfd: {}", value.0, value.1);
        Ok(())
    }
}

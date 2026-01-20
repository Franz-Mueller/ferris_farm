pub enum SensorError {
    ParseFloat(std::num::ParseFloatError),
    Database(std::io::Error),
}
impl std::fmt::Display for SensorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SensorError::ParseFloat(e) => write!(f, "invalid float: {e}"),
            SensorError::Database(e) => write!(f, "db error: {e}"),
        }
    }
}

trait Sensor {
    type Value;
    /// Processes a temperature value from a http request body recieved as string and returns the value ready for db
    fn process_measurement(&self, value: &str) -> Result<Self::Value, SensorError>;
    fn write_to_db(&self, value: &Self::Value) -> Result<(), SensorError>;
}

pub struct TempSensor;
pub struct HumSensor;
pub struct LuxSensor;

pub enum AnySensor {
    Temp(TempSensor),
    Hum(HumSensor),
    Lux(LuxSensor),
}

impl AnySensor {
    pub fn process(&self, body: &str) -> Result<(), SensorError> {
        match self {
            AnySensor::Temp(s) => {
                let value: f32 = s.process_measurement(body)?;
                s.write_to_db(&value)?;
            }
            AnySensor::Hum(s) => {
                let value: f32 = s.process_measurement(body)?;
                s.write_to_db(&value)?;
            }
            AnySensor::Lux(s) => {
                let value: (f32, f32) = s.process_measurement(body)?;
                s.write_to_db(&value)?;
            }
        }
        Ok(())
    }
}

impl Sensor for TempSensor {
    type Value = f32;

    fn process_measurement(&self, value: &str) -> Result<Self::Value, SensorError> {
        value.parse::<f32>().map_err(SensorError::ParseFloat)
    }
    fn write_to_db(&self, value: &f32) -> Result<(), SensorError> {
        println!("DB write temp: {value}");
        Ok(())
    }
}

impl Sensor for HumSensor {
    type Value = f32;

    fn process_measurement(&self, value: &str) -> Result<Self::Value, SensorError> {
        value.parse::<f32>().map_err(SensorError::ParseFloat)
    }
    fn write_to_db(&self, value: &f32) -> Result<(), SensorError> {
        println!("DB write hum: {value}");
        Ok(())
    }
}

impl Sensor for LuxSensor {
    type Value = (f32, f32);

    /// takes lux as &str and returns tuple (f32, f32) conatining lux and ppfd
    fn process_measurement(&self, value: &str) -> Result<Self::Value, SensorError> {
        let lux: f32 = value.parse::<f32>().map_err(SensorError::ParseFloat)?;
        // TODO should later be changed to user choosable value
        // 0,0185 is estimated value for LED Full-Spectrum
        Ok((lux, lux * 0.0185))
    }
    /// takes tuple (f32, f32) conatining lux and ppfd
    fn write_to_db(&self, value: &(f32, f32)) -> Result<(), SensorError> {
        println!("DB write lux: {} ppfd: {}", value.0, value.1);
        Ok(())
    }
}

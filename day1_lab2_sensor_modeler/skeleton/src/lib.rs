#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Unit {
    Celsius,
    Volt,
    Rpm,
}


impl Unit {
    pub fn parse(text: &str) -> Option<Self> {
        match text {
            "C" => Some(Self::Celsius), // Some : 값이 있다.
            "V" => Some(Self::Volt),
            "RPM"=> Some(Self::Rpm),
            _ => None,
        }
        // TODO: accept "C", "V", and "RPM".
        //let _ = text;
        //todo!("parse unit")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReadingStatus {
    Valid,
    Invalid,
}

impl ReadingStatus {
    pub fn parse(text: &str) -> Option<Self> {
        // TODO: accept "valid" and "invalid".
        let _ = text;
        todo!("parse status")
    }
}

#[derive(Debug, PartialEq)]
pub struct SensorReading {
    pub id: String,
    pub value: f32,
    pub unit: Unit,
    pub status: ReadingStatus,
}

pub fn parse_reading(line: &str) -> Option<SensorReading> {
    let mut parts = line.split(',');
    let id = parts.next()?.trim().to_string();
    let value = parts.next()?.trim().parse::<f32>().ok()?;
    let unit = Unit::parse(parts.next()?.trim())?;
    let status = ReadingStatus::parse(parts.next()?.trim())?;

    if parts.next().is_some() {return None;}
    Some(SensorReading {id, value, unit, status })
    // Expected format: "sensor-a,36.5,C,valid"
    //let _ = line;
    //todo!("parse CSV-like reading into SensorReading")
}

pub fn valid_average(readings: &[SensorReading]) -> Option<f32> {
    let mut sum = 0.0;
    let mut count = 0;

    for reading in readings {
        if reading.status == ReadingStatus::Valid {
            sum += reading.value;
            count += 1;
        }
    }
    if count == 0 {None} else {Some(sum/count as f32)}
    //let _ = readings;
    //todo!("average only readings with ReadingStatus::Valid")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_line() {
        let reading = parse_reading("temp-a,36.5,C,valid").expect("reading");
        assert_eq!(reading.id, "temp-a");
        assert_eq!(reading.value, 36.5);
        assert_eq!(reading.unit, Unit::Celsius);
        assert_eq!(reading.status, ReadingStatus::Valid);
    }

    #[test]
    fn rejects_unknown_unit() {
        assert_eq!(parse_reading("temp-a,36.5,K,valid"), None);
    }

    #[test]
    fn averages_only_valid_readings() {
        let readings = vec![
            parse_reading("a,10.0,V,valid").unwrap(),
            parse_reading("b,20.0,V,invalid").unwrap(),
            parse_reading("c,30.0,V,valid").unwrap(),
        ];
        assert_eq!(valid_average(&readings), Some(20.0));
    }
}

use std::{collections::HashMap, fmt::{Display, Formatter}};

fn main() {

    let input: &str = r"Hamburg;12.0
Bulawayo;8.9
Palembang;38.8
St. John's;15.2
Cracow;12.6
Bridgetown;26.9
Istanbul;6.2
Roseau;34.4
Conakry;31.2
Istanbul;23.0";

    println!("Hello, world!");
}

fn get_report(input: &str) -> HashMap<String, ReportWeatherStation> {
    let report: HashMap<String, ReportWeatherStation> = HashMap::new();



    report
}

struct WeatherStation {
    name: String,
    temperature: i64,
}

impl From<&str> for WeatherStation {
    fn from(line: &str) -> Self {
        let (name_as_str, temperature_as_str) = line.split_once(';').unwrap();
        let name: String = name_as_str.to_owned();
        let temperature: i64 = (temperature_as_str.parse::<f64>().unwrap() * 10.0) as i64;
        WeatherStation { name, temperature }
    }
}

struct ReportWeatherStation {
    name: String,
    min_temperature: i64,
    max_temperature: i64,
    no_readings: i64,
    sum_temperature: i64
}


impl Display for ReportWeatherStation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        let min: f64 = (self.min_temperature as f64) / 10.0;
        let max: f64 = (self.max_temperature as f64) / 10.0;


        write!(f, "{}={}/{}/{}", self.name, self.name, self.name, self.name)
    }
}



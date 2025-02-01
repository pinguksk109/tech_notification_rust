use crate::repository::weather_repository::WeatherRepository;

pub struct WeatherOutput {
    pub forecast: String,
}

impl WeatherOutput {
    pub fn new(forecast: String) -> Self {
        WeatherOutput { forecast }
    }
}

pub struct WeatherUsecase {
    weather_repository: WeatherRepository,
}

impl WeatherUsecase {
    pub fn new(weather_repository: WeatherRepository) -> Self {
        WeatherUsecase { weather_repository }
    }

    pub async fn handle(&self) -> Result<WeatherOutput, String> {
        let osaka_code = 270000;

        match self.weather_repository.fetch(osaka_code).await {
            Ok(response) => {
                let forecast = response[0]["timeSeries"][0]["areas"][0]["weathers"][0]
                    .as_str()
                    .ok_or_else(|| "Invalid forecast data".to_string())?;

                Ok(WeatherOutput::new(forecast.to_string()))
            }
            Err(err) => Err(format!("Error fetching weather data: {}", err)),
        }
    }
}
use bevy::prelude::*;
use rand::Rng;
use crate::prelude::*;
use crate::inventory_system::Equipment;

// ============================================================================
// WEATHER SYSTEM - The Long Dark inspired survival mechanics
// ============================================================================

#[derive(Resource, Clone)]
pub struct Weather {
    pub current_weather: WeatherType,
    pub temperature: f32, // In Celsius
    pub wind_speed: f32,  // km/h
    pub wind_direction: f32, // Degrees
    pub precipitation: f32, // 0.0 to 1.0
    pub visibility: f32, // 0.0 to 1.0
    pub time_until_change: f32, // Seconds
}

impl Default for Weather {
    fn default() -> Self {
        Weather {
            current_weather: WeatherType::Clear,
            temperature: 15.0,
            wind_speed: 5.0,
            wind_direction: 0.0,
            precipitation: 0.0,
            visibility: 1.0,
            time_until_change: 600.0, // 10 minutes
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum WeatherType {
    Clear,
    PartlyCloudy,
    Cloudy,
    LightRain,
    HeavyRain,
    LightSnow,
    HeavySnow,
    Blizzard,
    Fog,
    Thunderstorm,
    Sandstorm,
}

impl WeatherType {
    pub fn temperature_range(&self) -> (f32, f32) {
        match self {
            WeatherType::Clear => (10.0, 25.0),
            WeatherType::PartlyCloudy => (8.0, 20.0),
            WeatherType::Cloudy => (5.0, 15.0),
            WeatherType::LightRain => (5.0, 15.0),
            WeatherType::HeavyRain => (3.0, 12.0),
            WeatherType::LightSnow => (-5.0, 3.0),
            WeatherType::HeavySnow => (-15.0, 0.0),
            WeatherType::Blizzard => (-30.0, -10.0),
            WeatherType::Fog => (5.0, 15.0),
            WeatherType::Thunderstorm => (8.0, 18.0),
            WeatherType::Sandstorm => (25.0, 45.0),
        }
    }

    pub fn wind_speed_range(&self) -> (f32, f32) {
        match self {
            WeatherType::Clear => (0.0, 10.0),
            WeatherType::PartlyCloudy => (5.0, 15.0),
            WeatherType::Cloudy => (10.0, 20.0),
            WeatherType::LightRain => (10.0, 25.0),
            WeatherType::HeavyRain => (20.0, 40.0),
            WeatherType::LightSnow => (5.0, 15.0),
            WeatherType::HeavySnow => (15.0, 30.0),
            WeatherType::Blizzard => (40.0, 80.0),
            WeatherType::Fog => (0.0, 5.0),
            WeatherType::Thunderstorm => (30.0, 60.0),
            WeatherType::Sandstorm => (40.0, 70.0),
        }
    }

    pub fn visibility_modifier(&self) -> f32 {
        match self {
            WeatherType::Clear => 1.0,
            WeatherType::PartlyCloudy => 0.95,
            WeatherType::Cloudy => 0.85,
            WeatherType::LightRain => 0.7,
            WeatherType::HeavyRain => 0.5,
            WeatherType::LightSnow => 0.6,
            WeatherType::HeavySnow => 0.4,
            WeatherType::Blizzard => 0.1,
            WeatherType::Fog => 0.3,
            WeatherType::Thunderstorm => 0.4,
            WeatherType::Sandstorm => 0.2,
        }
    }

    pub fn precipitation_amount(&self) -> f32 {
        match self {
            WeatherType::LightRain | WeatherType::LightSnow => 0.3,
            WeatherType::HeavyRain | WeatherType::HeavySnow => 0.7,
            WeatherType::Blizzard | WeatherType::Thunderstorm => 0.9,
            _ => 0.0,
        }
    }
}

// ============================================================================
// TEMPERATURE AND SURVIVAL COMPONENTS
// ============================================================================

#[derive(Component)]
pub struct Temperature {
    pub core_temp: f32, // Body temperature in Celsius (normal ~37°C)
    pub feels_like: f32, // Accounting for wind chill/heat index
    pub warmth_bonus: f32, // From clothing/shelter
    pub cold_resistance: f32,
    pub heat_resistance: f32,
}

impl Default for Temperature {
    fn default() -> Self {
        Temperature {
            core_temp: 37.0,
            feels_like: 37.0,
            warmth_bonus: 0.0,
            cold_resistance: 1.0,
            heat_resistance: 1.0,
        }
    }
}

#[derive(Component)]
pub struct Wetness {
    pub wetness_level: f32, // 0.0 = dry, 1.0 = soaked
    pub drying_rate: f32,
}

impl Default for Wetness {
    fn default() -> Self {
        Wetness {
            wetness_level: 0.0,
            drying_rate: 0.01,
        }
    }
}

#[derive(Component)]
pub struct Shelter {
    pub protection_from_wind: f32, // 0.0 to 1.0
    pub protection_from_rain: f32,
    pub warmth_provided: f32,
}

// ============================================================================
// TIME OF DAY
// ============================================================================

#[derive(Resource)]
pub struct TimeOfDay {
    pub hour: f32, // 0.0 to 24.0
    pub day: i32,
    pub season: Season,
}

impl Default for TimeOfDay {
    fn default() -> Self {
        TimeOfDay {
            hour: 12.0,
            day: 1,
            season: Season::Spring,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

impl Season {
    pub fn temperature_modifier(&self) -> f32 {
        match self {
            Season::Spring => 0.0,
            Season::Summer => 10.0,
            Season::Autumn => -5.0,
            Season::Winter => -15.0,
        }
    }

    pub fn next(&self) -> Season {
        match self {
            Season::Spring => Season::Summer,
            Season::Summer => Season::Autumn,
            Season::Autumn => Season::Winter,
            Season::Winter => Season::Spring,
        }
    }
}

// ============================================================================
// LIGHTING BASED ON TIME
// ============================================================================

#[derive(Component)]
pub struct AmbientLight {
    pub brightness: f32, // 0.0 to 1.0
}

// ============================================================================
// WEATHER PLUGIN
// ============================================================================

pub struct WeatherPlugin;

impl Plugin for WeatherPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Weather::default())
            .insert_resource(TimeOfDay::default())
            .add_systems(
                Update,
                (
                    weather_update_system,
                    time_of_day_system,
                    temperature_effect_system,
                    wetness_system,
                    hypothermia_system,
                    hyperthermia_system,
                    wind_chill_system,
                )
                    .run_if(bevy::time::common_conditions::on_timer(
                        bevy::utils::Duration::from_secs(2),
                    )),
            )
            .add_systems(Update, day_night_lighting_system);
    }
}

// System to update weather conditions
fn weather_update_system(mut weather: ResMut<Weather>, time: Res<Time>) {
    weather.time_until_change -= time.delta_seconds();

    if weather.time_until_change <= 0.0 {
        let mut rng = rand::thread_rng();

        // Pick new weather
        weather.current_weather = match rng.gen_range(0..10) {
            0 => WeatherType::Clear,
            1 => WeatherType::PartlyCloudy,
            2 => WeatherType::Cloudy,
            3 => WeatherType::LightRain,
            4 => WeatherType::HeavyRain,
            5 => WeatherType::LightSnow,
            6 => WeatherType::HeavySnow,
            7 => WeatherType::Fog,
            8 => WeatherType::Thunderstorm,
            9 => WeatherType::Blizzard,
            _ => WeatherType::Clear,
        };

        // Set temperature
        let (min_temp, max_temp) = weather.current_weather.temperature_range();
        weather.temperature = rng.gen_range(min_temp..max_temp);

        // Set wind
        let (min_wind, max_wind) = weather.current_weather.wind_speed_range();
        weather.wind_speed = rng.gen_range(min_wind..max_wind);
        weather.wind_direction = rng.gen_range(0.0..360.0);

        // Set precipitation and visibility
        weather.precipitation = weather.current_weather.precipitation_amount();
        weather.visibility = weather.current_weather.visibility_modifier();

        // Set next change time (5-15 minutes)
        weather.time_until_change = rng.gen_range(300.0..900.0);
    }
}

// System to advance time
fn time_of_day_system(
    mut time_of_day: ResMut<TimeOfDay>,
    time: Res<Time>,
) {
    // Advance time (1 real second = 1 game minute)
    time_of_day.hour += time.delta_seconds() / 60.0;

    if time_of_day.hour >= 24.0 {
        time_of_day.hour -= 24.0;
        time_of_day.day += 1;

        // Change season every 30 days
        if time_of_day.day % 30 == 0 {
            time_of_day.season = time_of_day.season.next();
        }
    }
}

// System to apply temperature effects to characters
fn temperature_effect_system(
    mut query: Query<(&mut Temperature, &Equipment, Option<&Shelter>)>,
    weather: Res<Weather>,
    time_of_day: Res<TimeOfDay>,
) {
    let ambient_temp = weather.temperature + time_of_day.season.temperature_modifier();

    for (mut temp, equipment, shelter) in query.iter_mut() {
        // Calculate warmth from equipment
        temp.warmth_bonus = equipment.total_warmth();

        // Calculate shelter bonus
        let shelter_warmth = if let Some(shelter) = shelter {
            shelter.warmth_provided
        } else {
            0.0
        };

        // Calculate effective temperature
        let effective_temp = ambient_temp + temp.warmth_bonus + shelter_warmth;

        // Move core temperature towards effective temperature
        let temp_diff = effective_temp - temp.core_temp;
        temp.core_temp += temp_diff * 0.01; // Slow change

        // Clamp core temperature
        temp.core_temp = temp.core_temp.clamp(25.0, 42.0);
    }
}

// System to calculate wind chill
fn wind_chill_system(
    mut query: Query<&mut Temperature>,
    weather: Res<Weather>,
) {
    for mut temp in query.iter_mut() {
        // Wind chill formula (simplified)
        if weather.wind_speed > 5.0 {
            let wind_chill = -1.5 * (weather.wind_speed / 10.0);
            temp.feels_like = temp.core_temp + wind_chill;
        } else {
            temp.feels_like = temp.core_temp;
        }
    }
}

// System to handle wetness from rain
fn wetness_system(
    mut query: Query<(&mut Wetness, Option<&Shelter>)>,
    weather: Res<Weather>,
) {
    for (mut wetness, shelter) in query.iter_mut() {
        // Get wet in rain
        if weather.precipitation > 0.0 {
            let protection = if let Some(shelter) = shelter {
                shelter.protection_from_rain
            } else {
                0.0
            };

            let wet_amount = weather.precipitation * (1.0 - protection) * 0.01;
            wetness.wetness_level = (wetness.wetness_level + wet_amount).min(1.0);
        }

        // Dry off
        wetness.wetness_level = (wetness.wetness_level - wetness.drying_rate).max(0.0);
    }
}

// System to apply hypothermia damage
fn hypothermia_system(
    mut query: Query<(&Temperature, &mut PhysicalBody, &Wetness)>,
) {
    for (temp, mut body, wetness) in query.iter_mut() {
        // Hypothermia threshold
        if temp.core_temp < 35.0 {
            // Mild hypothermia
            body.attributes.health -= 1;
        }

        if temp.core_temp < 32.0 {
            // Severe hypothermia
            body.attributes.health -= 5;
        }

        // Being wet makes you colder
        if wetness.wetness_level > 0.5 && temp.core_temp < 36.0 {
            body.attributes.health -= 2;
        }
    }
}

// System to apply hyperthermia damage
fn hyperthermia_system(
    mut query: Query<(&Temperature, &mut PhysicalBody)>,
) {
    for (temp, mut body) in query.iter_mut() {
        // Heat exhaustion
        if temp.core_temp > 39.0 {
            body.attributes.health -= 1;
        }

        // Heat stroke
        if temp.core_temp > 41.0 {
            body.attributes.health -= 5;
        }
    }
}

// System to update lighting based on time of day
fn day_night_lighting_system(
    time_of_day: Res<TimeOfDay>,
    mut ambient_query: Query<&mut AmbientLight>,
) {
    let brightness = calculate_brightness(time_of_day.hour);

    for mut ambient in ambient_query.iter_mut() {
        ambient.brightness = brightness;
    }
}

fn calculate_brightness(hour: f32) -> f32 {
    // Simple brightness curve
    // Night: 0-6, 20-24 = dark
    // Dawn: 6-8 = getting brighter
    // Day: 8-18 = bright
    // Dusk: 18-20 = getting darker

    if hour < 6.0 || hour > 20.0 {
        0.2 // Night
    } else if hour < 8.0 {
        // Dawn
        0.2 + (hour - 6.0) * 0.4
    } else if hour < 18.0 {
        1.0 // Day
    } else {
        // Dusk
        1.0 - (hour - 18.0) * 0.4
    }
}

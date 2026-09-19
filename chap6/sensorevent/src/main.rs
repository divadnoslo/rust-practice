enum SensorEvent {
    Temperature(f64),                     // Temperature in Celsius
    Humidity(f64),                        // Humidity percentage
    BatteryStatus(u8),                    // Battery percentage (0-100)
    Ping,                                 // Keep-alive heartbeat (no data)
    Error { code: u32, message: String }, // Error details
}

fn main() {
    
    let mut events = vec![
        SensorEvent::Ping, 
        SensorEvent::Ping, 
        SensorEvent::Temperature(27.0), 
        SensorEvent::Ping, 
        SensorEvent::BatteryStatus(22), 
        SensorEvent::Ping, 
        SensorEvent::Ping, 
        SensorEvent::Humidity(49.0), 
        SensorEvent::Ping, 
        SensorEvent::Temperature(36.0), 
        SensorEvent::Ping, 
        SensorEvent::Ping, 
        SensorEvent::Humidity(55.0), 
        SensorEvent::Ping, 
        SensorEvent::Ping, 
        SensorEvent::BatteryStatus(15), 
    ].into_iter();

    // while let keeps popping elements off as long as .next() returns Some(event)
    while let Some(event) = events.next() {
        check_high_temperature(&event);
        check_high_humidity(&event);
        process_battery(&event);
    }

}

fn check_high_temperature(event: &SensorEvent) {
    if let SensorEvent::Temperature(temp) = event {
        if *temp > 35.0 {
            println!("WARNING: High temperature detected: {}°C!", temp);
        }
    }
}

fn check_high_humidity(event: &SensorEvent) {
    if let SensorEvent::Humidity(reading) = event {
        if *reading > 50.0 {
            println!("WARNING: High humidity detected: {} humids", reading);
        }
    }
}

fn process_battery(event: &SensorEvent) {
    if let SensorEvent::BatteryStatus(level) = event {
        if *level < 20 {
            println!("CRITICAL: Low battery ({level}%)!");
        } else {
            println!("Battery status is fine");
        }
    }
}
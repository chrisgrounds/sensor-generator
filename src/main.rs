use csv::Writer;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::Serialize;
use std::error::Error;
use std::fs::File;
use strum::EnumCount;
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

#[derive(Debug, EnumCountMacro, EnumIter, Serialize)]
pub enum EventType {
    SensorTriggered,
    SensorAcknowledged,
    SensorOff,
}

#[derive(Debug, Serialize)]
pub struct EventData {
    pub event_type: EventType,
    pub timestamp: u64,
    pub correlation_id: u64,
}

impl Distribution<EventData> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> EventData {
        let (timestamp, correlation_id) = rng.gen();
        let event_type = match rng.gen_range(0..=EventType::COUNT) {
            0 => EventType::SensorAcknowledged,
            1 => EventType::SensorTriggered,
            _ => EventType::SensorOff,
        };

        EventData {
            event_type,
            timestamp,
            correlation_id,
        }
    }
}

fn write_event_to_csv(wtr: &mut Writer<File>, event_data: EventData) -> Result<(), Box<dyn Error>> {
    wtr.serialize(&event_data)?;
    wtr.flush()?;
    Ok(())
}

fn main() {
    let total_events = 1000;
    let mut rng = rand::thread_rng();
    let mut wtr = Writer::from_path("random_data.csv").unwrap();

    for _ in 0..total_events {
        let random_event: EventData = rng.gen();
        println!("{:?}", random_event);

        write_event_to_csv(&mut wtr, random_event).unwrap();
    }

    println!("Done!");
}
